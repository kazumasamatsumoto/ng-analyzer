mod ast;
mod analyzers;
mod cli;
mod config;
mod output;
mod parsers;
mod search;

use crate::analyzers::AnalysisEngine;
use crate::cli::{Cli, Commands, AnalysisConfig};
use crate::config::Config;
use crate::output::create_formatter;
use crate::parsers::ProjectParser;
use crate::search::{SearchConfig, SimpleSearchEngine, SearchType};
use crate::analyzers::dependency_graph::DependencyGraphAnalyzer;
use crate::output::graph::GraphFormatter;
use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;
use std::fs;

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
        Commands::Search {
            path,
            keyword,
            file_type,
            file_pattern,
            case_sensitive,
            line_numbers,
            context,
            output,
            search_type: _,
            regex: _,
            html_class: _,
            html_text: _,
            function_name: _,
            structural: _,
        } => {
            let search_config = SearchConfig::new(
                path,
                keyword,
                Some(file_type),
                file_pattern,
                case_sensitive,
                line_numbers,
                context,
                output,
                cli.verbose,
            );
            
            // TODO: 検索タイプの処理は後で実装
            // 今は基本的な検索のみ実装
            run_search(search_config).await?;
        }
        Commands::Graph {
            path,
            format,
            output,
            circular,
            orphaned,
            depth,
            top_count,
            extensions,
            exclude_external,
        } => {
            run_graph_analysis(
                path,
                format,
                output,
                circular,
                orphaned,
                depth,
                top_count,
                extensions,
                exclude_external,
                cli.verbose,
                cli.quiet,
            ).await?;
        }
    }

    if !cli.quiet {
        let duration = start_time.elapsed();
        println!("Analysis completed in {:.2}s", duration.as_secs_f64());
    }

    Ok(())
}

async fn run_analysis(config: AnalysisConfig) -> Result<()> {
    if config.verbose {
        println!("🔍 Starting Angular project analysis...");
        println!("📁 Analyzing path: {}", config.path.display());
    }

    let parser = ProjectParser::new();
    let project = parser.parse_project(&config.path).await?;

    if config.verbose {
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
                .filter(|issue| matches!(issue.severity, ast::Severity::Error | ast::Severity::Warning))
                .count()
        })
        .sum();

    match config.output_format {
        crate::cli::args::OutputFormat::Json => {
            let formatter = create_formatter("json")?;
            let output = formatter.format(&results)?;
            println!("{}", output);
        }
        crate::cli::args::OutputFormat::Table => {
            let formatter = create_formatter("table")?;
            let output = formatter.format(&results)?;
            println!("{}", output);
        }
        crate::cli::args::OutputFormat::Html => {
            let formatter = create_formatter("html")?;
            let output = formatter.format(&results)?;
            if let Some(output_dir) = &config.output_dir {
                std::fs::create_dir_all(output_dir)?;
                let output_file = output_dir.join("analysis-report.html");
                std::fs::write(&output_file, output)?;
                if config.verbose {
                    println!("📄 HTML report generated: {}", output_file.display());
                }
            } else {
                println!("{}", output);
            }
        }
    }

    if config.verbose {
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

async fn run_search(config: SearchConfig) -> Result<()> {
    let _engine = SimpleSearchEngine::new(
        config.keyword.clone(),
        config.case_sensitive,
        config.line_numbers,
        config.context,
    );
    
    // TODO: この部分は後で実装する必要があります
    // 今は仮の実装として空のベクトルを返します
    let results: Vec<crate::search::simple::SearchResult> = Vec::new();
    
    if results.is_empty() {
        if config.verbose {
            println!("⚠️  No matches found");
        }
        return Ok(());
    }
    
    if config.verbose {
        let total_matches: usize = results.iter().map(|r| r.total_matches()).sum();
        println!("🔍 Found {} matches in {} files", total_matches, results.len());
    }
    
    match config.output_format.as_str() {
        "json" => {
            let json_output = serde_json::to_string_pretty(&results)?;
            println!("{}", json_output);
        }
        "table" => {
            print_table_format(&results, &config);
        }
        _ => {
            print_simple_format(&results, &config);
        }
    }
    
    Ok(())
}

async fn run_graph_analysis(
    path: PathBuf,
    format: String,
    output: Option<PathBuf>,
    _circular: bool,
    _orphaned: bool,
    _depth: bool,
    _top_count: u32,
    _extensions: Option<Vec<String>>,
    _exclude_external: bool,
    _verbose: bool,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("🔍 TypeScript依存関係グラフ分析を開始しています...");
        println!("📁 分析対象パス: {}", path.display());
    }

    let analyzer = DependencyGraphAnalyzer::new();
    let graph = analyzer.analyze_project(&path).await?;

    if !quiet {
        println!(
            "📊 {}個のファイルと{}個の依存関係を発見しました",
            graph.files.len(),
            graph.dependencies.len()
        );
    }

    let analysis = analyzer.analyze_dependencies(&graph)?;
    
    if !quiet {
        println!("🔍 依存関係分析を実行しています...");
        
        if !analysis.circular_dependencies.is_empty() {
            println!("⚠️  {}個の循環依存を発見しました", analysis.circular_dependencies.len());
        }
        
        if !analysis.orphaned_files.is_empty() {
            println!("🔍 {}個の孤立ファイルを発見しました", analysis.orphaned_files.len());
        }
    }

    let formatter = GraphFormatter::new();
    let output_content = match format.as_str() {
        "dot" => formatter.format_dot(&graph, &analysis)?,
        "mermaid" => formatter.format_mermaid(&graph, &analysis)?,
        "json" => formatter.format_json(&graph, &analysis)?,
        "table" => formatter.format_table(&graph, &analysis)?,
        _ => return Err(anyhow::anyhow!("サポートされていない出力形式: {}", format)),
    };

    if let Some(output_path) = output {
        fs::write(&output_path, &output_content)?;
        if !quiet {
            println!("📄 グラフが出力されました: {}", output_path.display());
        }
    } else {
        println!("{}", output_content);
    }

    if !quiet {
        println!("\n📈 分析サマリー:");
        println!("   総ファイル数: {}", graph.files.len());
        println!("   総依存関係数: {}", graph.dependencies.len());
        println!("   循環依存数: {}", analysis.circular_dependencies.len());
        println!("   孤立ファイル数: {}", analysis.orphaned_files.len());
        
        if !analysis.most_imported_files.is_empty() {
            println!("   最もインポートされているファイル:");
            for (file_path, count) in analysis.most_imported_files.iter().take(3) {
                println!("     - {} ({}回)", file_path, count);
            }
        }
    }

    Ok(())
}

fn print_simple_format(results: &[crate::search::simple::SearchResult], config: &SearchConfig) {
    for result in results {
        println!("\n📄 {}", result.file_path);
        println!("   {} matches found", result.total_matches());
        
        for search_match in &result.matches {
            if config.line_numbers {
                println!("   {}:", search_match.line_number);
            }
            
            // Print context before
            for context_line in &search_match.context_before {
                println!("     {}", context_line);
            }
            
            // Print the matching line
            println!("   → {}", search_match.line_content);
            
            // Print context after
            for context_line in &search_match.context_after {
                println!("     {}", context_line);
            }
        }
    }
}

fn print_table_format(results: &[crate::search::simple::SearchResult], config: &SearchConfig) {
    println!("{:<40} {:<6} {:<80}", "File", "Line", "Content");
    println!("{}", "-".repeat(126));
    
    for result in results {
        for search_match in &result.matches {
            let file = if result.file_path.len() > 35 {
                format!("...{}", &result.file_path[result.file_path.len()-32..])
            } else {
                result.file_path.clone()
            };
            
            let line = if config.line_numbers {
                search_match.line_number.to_string()
            } else {
                "-".to_string()
            };
            
            let content = if search_match.line_content.len() > 75 {
                format!("{}...", &search_match.line_content[..72])
            } else {
                search_match.line_content.clone()
            };
            
            println!("{:<40} {:<6} {:<80}", 
                     file, line, content);
        }
    }
}