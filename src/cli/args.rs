use std::path::PathBuf;
use crate::ast::Severity;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Html,
    Table,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub path: PathBuf,
    pub output_format: OutputFormat,
    pub output_dir: Option<PathBuf>,
    pub analyzers: Vec<String>,
    pub severity: Severity,
    #[allow(dead_code)]
    pub max_complexity: u32,
    #[allow(dead_code)]
    pub max_depth: u32,
    #[allow(dead_code)]
    pub config_file: Option<PathBuf>,
    #[allow(dead_code)]
    pub verbose: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./src"),
            analyzers: vec!["component".to_string()],
            output_format: OutputFormat::Json,
            output_dir: Some(PathBuf::from("./reports")),
            severity: Severity::Info,
            max_complexity: 10,
            max_depth: 5,
            config_file: None,
            verbose: false,
        }
    }
}

impl AnalysisConfig {
    #[allow(dead_code)]
    pub fn from_component_args(
        path: PathBuf,
        max_complexity: u32,
        depth: u32,
        output: String,
        errors_only: bool,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let output_format = match output.as_str() {
            "html" => OutputFormat::Html,
            "table" => OutputFormat::Table,
            _ => OutputFormat::Json,
        };
        
        Self {
            path,
            analyzers: vec!["component".to_string()],
            output_format,
            severity: if errors_only { Severity::Warning } else { Severity::Info },
            max_complexity,
            max_depth: depth,
            verbose,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_deps_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let output_format = match format.as_str() {
            "html" => OutputFormat::Html,
            "table" => OutputFormat::Table,
            _ => OutputFormat::Json,
        };
        
        Self {
            path,
            analyzers: vec!["dependency".to_string()],
            output_format,
            verbose,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_state_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let output_format = match format.as_str() {
            "html" => OutputFormat::Html,
            "table" => OutputFormat::Table,
            _ => OutputFormat::Json,
        };
        
        Self {
            path,
            analyzers: vec!["state".to_string()],
            output_format,
            verbose,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_performance_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let output_format = match format.as_str() {
            "html" => OutputFormat::Html,
            "table" => OutputFormat::Table,
            _ => OutputFormat::Json,
        };
        
        Self {
            path,
            analyzers: vec!["performance".to_string()],
            output_format,
            verbose,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_audit_args(
        path: PathBuf,
        full: bool,
        analyzers: Option<Vec<String>>,
        config: Option<PathBuf>,
        output_dir: PathBuf,
        formats: Vec<String>,
        severity: String,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let analyzers = if full {
            vec![
                "component".to_string(),
                "dependency".to_string(),
                "state".to_string(),
                "performance".to_string(),
            ]
        } else {
            analyzers.unwrap_or_else(|| vec!["component".to_string()])
        };

        let severity_threshold = match severity.to_lowercase().as_str() {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            _ => Severity::Info,
        };

        // 最初のフォーマットを使用（複数対応は将来的に追加）
        let output_format = match formats.first().map(|s| s.as_str()) {
            Some("html") => OutputFormat::Html,
            Some("table") => OutputFormat::Table,
            _ => OutputFormat::Json,
        };

        Self {
            path,
            analyzers,
            output_format,
            output_dir: Some(output_dir),
            severity: severity_threshold,
            config_file: config,
            verbose,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn should_include_issue(&self, severity: &Severity) -> bool {
        match (&self.severity, severity) {
            (Severity::Error, Severity::Error) => true,
            (Severity::Warning, Severity::Error | Severity::Warning) => true,
            (Severity::Info, _) => true,
            _ => false,
        }
    }
    
    #[allow(dead_code)]
    pub fn from_search_args(
        path: PathBuf,
        _keyword: String,
        _file_type: String,
        _file_pattern: Option<String>,
        _case_sensitive: bool,
        _line_numbers: bool,
        _context: u32,
        output: String,
        verbose: bool,
        _quiet: bool,
    ) -> Self {
        let output_format = match output.as_str() {
            "html" => OutputFormat::Html,
            "table" => OutputFormat::Table,
            _ => OutputFormat::Json,
        };
        
        Self {
            path,
            analyzers: vec!["search".to_string()],
            output_format,
            verbose,
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
pub fn parse_severity(s: &str) -> Result<Severity, String> {
    match s.to_lowercase().as_str() {
        "error" => Ok(Severity::Error),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        _ => Err(format!("Invalid severity: {}. Use 'error', 'warning', or 'info'", s)),
    }
}