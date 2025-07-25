use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ng-analyzer")]
#[command(about = "A powerful Angular project analyzer built with Rust")]
#[command(version = "0.1.0")]
#[command(author = "Angular Analysis Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze Angular components for best practices and performance
    Component {
        /// Path to analyze
        path: PathBuf,
        
        /// Maximum complexity threshold
        #[arg(long, default_value = "10")]
        max_complexity: u32,
        
        /// Maximum analysis depth
        #[arg(short, long, default_value = "5")]
        depth: u32,
        
        /// Output format (json, table, html)
        #[arg(short, long, default_value = "json")]
        output: String,
        
        /// Show only errors and warnings
        #[arg(long)]
        errors_only: bool,
    },
    
    /// Analyze dependencies and architectural patterns
    Deps {
        /// Path to analyze
        path: PathBuf,
        
        /// Check for circular dependencies
        #[arg(long)]
        circular: bool,
        
        /// Find unused dependencies
        #[arg(long)]
        unused: bool,
        
        /// Analyze dependency depth
        #[arg(long)]
        depth: bool,
        
        /// Output format (json, table, html)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Analyze state management patterns and reactive programming
    State {
        /// Path to analyze
        path: PathBuf,
        
        /// Analyze NgRx patterns
        #[arg(long)]
        ngrx: bool,
        
        /// Check for memory leaks and subscription management
        #[arg(long)]
        subscriptions: bool,
        
        /// Analyze change detection impact
        #[arg(long)]
        change_detection: bool,
        
        /// Output format (json, table, html)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// Analyze performance implications and optimization opportunities
    Performance {
        /// Path to analyze
        path: PathBuf,
        
        /// Check bundle size impact
        #[arg(long)]
        bundle_size: bool,
        
        /// Analyze lazy loading opportunities
        #[arg(long)]
        lazy_loading: bool,
        
        /// Check for memory leak risks
        #[arg(long)]
        memory_leaks: bool,
        
        /// Output format (json, table, html)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// Run comprehensive audit with all analyzers
    Audit {
        /// Path to analyze
        path: PathBuf,
        
        /// Run all analyzers
        #[arg(long)]
        full: bool,
        
        /// Specific analyzers to run (comma-separated)
        #[arg(long, value_delimiter = ',')]
        analyzers: Option<Vec<String>>,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        /// Output directory for reports
        #[arg(short, long, default_value = "./reports")]
        output_dir: PathBuf,
        
        /// Output formats (json, html, table)
        #[arg(long, value_delimiter = ',', default_values = ["json"])]
        formats: Vec<String>,
        
        /// Severity threshold (error, warning, info)
        #[arg(long, default_value = "info")]
        severity: String,
    },
    
    /// Initialize configuration file
    Init {
        /// Output configuration file path
        #[arg(short, long, default_value = ".ng-analyzer.json")]
        output: PathBuf,
        
        /// Configuration profile (strict, recommended, relaxed)
        #[arg(short, long, default_value = "recommended")]
        profile: String,
    },
    
    /// List available analyzers and rules
    List {
        /// Show detailed information about analyzers
        #[arg(long)]
        details: bool,
        
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
    },
    
    /// Search for keywords in project files
    Search {
        /// Path to search in
        path: PathBuf,
        
        /// Keyword to search for
        #[arg(short, long)]
        keyword: String,
        
        /// File types to search in (html, ts, js, all)
        #[arg(short, long, default_value = "all")]
        file_type: String,
        
        /// Specific file pattern to search in
        #[arg(long)]
        file_pattern: Option<String>,
        
        /// Case sensitive search
        #[arg(long)]
        case_sensitive: bool,
        
        /// Show line numbers
        #[arg(long)]
        line_numbers: bool,
        
        /// Show context lines around matches
        #[arg(short, long, default_value = "0")]
        context: u32,
        
        /// Output format (json, table, simple)
        #[arg(short, long, default_value = "simple")]
        output: String,
        
        /// Search type (simple, regex, html-class, html-text, function, structural)
        #[arg(long, default_value = "simple")]
        search_type: String,
        
        /// Use regular expressions for search
        #[arg(long)]
        regex: bool,
        
        /// Search for HTML class names
        #[arg(long)]
        html_class: bool,
        
        /// Search for HTML text content
        #[arg(long)]
        html_text: bool,
        
        /// Search for function names
        #[arg(long)]
        function_name: bool,
        
        /// Structural search pattern (for complex patterns)
        #[arg(long)]
        structural: Option<String>,
    },
    
    /// Analyze TypeScript import/export relationships and generate dependency graphs
    Graph {
        /// Path to analyze
        path: PathBuf,
        
        /// Output format (dot, mermaid, json, table)
        #[arg(short, long, default_value = "mermaid")]
        format: String,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Include circular dependency analysis
        #[arg(long)]
        circular: bool,
        
        /// Include orphaned file detection
        #[arg(long)]
        orphaned: bool,
        
        /// Include dependency depth analysis
        #[arg(long)]
        depth: bool,
        
        /// Maximum number of files to show in top lists
        #[arg(long, default_value = "10")]
        top_count: u32,
        
        /// Filter files by extension (ts, js, tsx, jsx)
        #[arg(long, value_delimiter = ',')]
        extensions: Option<Vec<String>>,
        
        /// Exclude node_modules and other directories
        #[arg(long)]
        exclude_external: bool,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}