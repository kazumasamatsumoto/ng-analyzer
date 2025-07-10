mod ast;
mod analyzers;
mod cli;
mod config;
mod output;
mod parsers;

use crate::analyzers::AnalysisEngine;
use crate::cli::{Cli, Commands, AnalysisConfig};
use crate::config::Config;
use crate::output::{create_multi_formatter, create_formatter};
use crate::parsers::ProjectParser;
use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();

    let start_time = Instant::now();

    match cli.command {
        Commands::Component {
            path,
            max_complexity,
            depth,
            output,
            errors_only,
        } => {
            let config = AnalysisConfig::from_component_args(
                path,
                max_complexity,
                depth,
                output,
                errors_only,
                cli.verbose,
                cli.quiet,
            );
            run_analysis(config).await?;
        }
        Commands::Deps { path, format, .. } => {
            let config = AnalysisConfig::from_deps_args(path, format, cli.verbose, cli.quiet);
            run_analysis(config).await?;
        }
        Commands::State { path, format, .. } => {
            let config = AnalysisConfig::from_state_args(path, format, cli.verbose, cli.quiet);
            run_analysis(config).await?;
        }
        Commands::Performance { path, format, .. } => {
            let config = AnalysisConfig::from_performance_args(path, format, cli.verbose, cli.quiet);
            run_analysis(config).await?;
        }
        Commands::Audit {
            path,
            full,
            analyzers,
            config,
            output_dir,
            formats,
            severity,
        } => {
            let analysis_config = AnalysisConfig::from_audit_args(
                path,
                full,
                analyzers,
                config,
                output_dir,
                formats,
                severity,
                cli.verbose,
                cli.quiet,
            );
            run_analysis(analysis_config).await?;
        }
        Commands::Init { output, profile } => {
            initialize_config(output, &profile)?;
        }
        Commands::List { details, category } => {
            list_analyzers(details, category)?;
        }
    }

    if !cli.quiet {
        let duration = start_time.elapsed();
        println!("Analysis completed in {:.2}s", duration.as_secs_f64());
    }

    Ok(())
}

async fn run_analysis(config: AnalysisConfig) -> Result<()> {
    if !config.quiet {
        println!("🔍 Starting Angular project analysis...");
        println!("📁 Analyzing path: {}", config.path.display());
    }

    let parser = ProjectParser::new();
    let project = parser.parse_project(&config.path).await?;

    if !config.quiet {
        println!(
            "📊 Found {} components, {} services, {} modules",
            project.components.len(),
            project.services.len(),
            project.modules.len()
        );
    }

    let engine = AnalysisEngine::new();
    let results = engine.run_analysis(&project, &config.analyzers).await?;

    if results.is_empty() {
        println!("⚠️  No analysis results generated");
        return Ok(());
    }

    let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();
    let filtered_issues: usize = results
        .iter()
        .map(|r| {
            r.issues
                .iter()
                .filter(|issue| config.should_include_issue(&issue.severity))
                .count()
        })
        .sum();

    if config.output_formats.len() == 1 && config.output_formats[0] == "json" {
        let formatter = create_formatter("json")?;
        let output = formatter.format(&results)?;
        println!("{}", output);
    } else if config.output_formats.len() == 1 && config.output_formats[0] == "table" {
        let formatter = create_formatter("table")?;
        let output = formatter.format(&results)?;
        println!("{}", output);
    } else {
        let multi_formatter = create_multi_formatter(&config.output_formats)?;
        multi_formatter.format_all(&results, &config.output_dir)?;

        if !config.quiet {
            println!("📄 Reports generated in: {}", config.output_dir.display());
            for format in &config.output_formats {
                println!("  - analysis-report.{}", format);
            }
        }
    }

    if !config.quiet {
        println!("\n📈 Analysis Summary:");
        println!("   Total issues found: {}", total_issues);
        println!("   Issues shown: {}", filtered_issues);

        if filtered_issues > 0 {
            let error_count = results
                .iter()
                .flat_map(|r| &r.issues)
                .filter(|issue| matches!(issue.severity, ast::Severity::Error))
                .count();
            let warning_count = results
                .iter()
                .flat_map(|r| &r.issues)
                .filter(|issue| matches!(issue.severity, ast::Severity::Warning))
                .count();

            if error_count > 0 {
                println!("   ❌ Errors: {}", error_count);
            }
            if warning_count > 0 {
                println!("   ⚠️  Warnings: {}", warning_count);
            }
        }

        let recommendation_count: usize = results.iter().map(|r| r.recommendations.len()).sum();
        if recommendation_count > 0 {
            println!("   💡 Recommendations: {}", recommendation_count);
        }
    }

    Ok(())
}

fn initialize_config(output_path: PathBuf, profile: &str) -> Result<()> {
    if output_path.exists() {
        println!("⚠️  Configuration file already exists at: {}", output_path.display());
        println!("   Use --force to overwrite (not implemented yet)");
        return Ok(());
    }

    Config::create_default_config_file(&output_path, profile)?;

    println!("✅ Configuration file created: {}", output_path.display());
    println!("   Profile: {}", profile);
    println!("   You can now customize the rules and settings in this file.");

    Ok(())
}

fn list_analyzers(details: bool, category: Option<String>) -> Result<()> {
    use crate::config::rules::{get_all_rule_definitions, get_available_categories, get_rules_by_category};

    if let Some(cat) = category {
        let rules = get_rules_by_category(&cat);
        if rules.is_empty() {
            println!("❌ No rules found for category: {}", cat);
            return Ok(());
        }

        println!("📋 Rules in category '{}':", cat);
        for rule in rules {
            println!("   • {}", rule.name);
            if details {
                println!("     Description: {}", rule.description);
                println!("     Default severity: {}", rule.default_severity);
                if !rule.configurable_options.is_empty() {
                    println!("     Configurable options:");
                    for option in &rule.configurable_options {
                        println!("       - {}: {} (default: {})", 
                            option.name, option.description, option.default_value);
                    }
                }
                println!();
            }
        }
    } else {
        let categories = get_available_categories();
        println!("📋 Available categories:");
        for category in &categories {
            println!("   • {}", category);
        }

        if details {
            println!("\n📋 Available analyzers:");
            println!("   • component - Analyzes Angular components for best practices and performance");
            println!("   • dependency - Analyzes dependency relationships and circular dependencies");
            println!("   • state - Analyzes state management patterns and reactive programming");
            println!("   • performance - Analyzes performance implications and optimization opportunities");

            println!("\n📋 All available rules:");
            let rules = get_all_rule_definitions();
            for rule in rules {
                println!("   • {} ({})", rule.name, rule.category);
                println!("     {}", rule.description);
                println!("     Default severity: {}", rule.default_severity);
                println!();
            }
        }
    }

    Ok(())
}