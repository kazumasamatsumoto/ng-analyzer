use std::path::PathBuf;
use crate::ast::Severity;

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub path: PathBuf,
    pub analyzers: Vec<String>,
    pub output_formats: Vec<String>,
    pub output_dir: PathBuf,
    pub severity_threshold: Severity,
    pub max_complexity: u32,
    pub max_depth: u32,
    pub config_file: Option<PathBuf>,
    pub verbose: bool,
    pub quiet: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./src"),
            analyzers: vec!["component".to_string()],
            output_formats: vec!["json".to_string()],
            output_dir: PathBuf::from("./reports"),
            severity_threshold: Severity::Info,
            max_complexity: 10,
            max_depth: 5,
            config_file: None,
            verbose: false,
            quiet: false,
        }
    }
}

impl AnalysisConfig {
    pub fn from_component_args(
        path: PathBuf,
        max_complexity: u32,
        depth: u32,
        output: String,
        errors_only: bool,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            analyzers: vec!["component".to_string()],
            output_formats: vec![output],
            severity_threshold: if errors_only { Severity::Warning } else { Severity::Info },
            max_complexity,
            max_depth: depth,
            verbose,
            quiet,
            ..Default::default()
        }
    }

    pub fn from_deps_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            analyzers: vec!["dependency".to_string()],
            output_formats: vec![format],
            verbose,
            quiet,
            ..Default::default()
        }
    }

    pub fn from_state_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            analyzers: vec!["state".to_string()],
            output_formats: vec![format],
            verbose,
            quiet,
            ..Default::default()
        }
    }

    pub fn from_performance_args(
        path: PathBuf,
        format: String,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            analyzers: vec!["performance".to_string()],
            output_formats: vec![format],
            verbose,
            quiet,
            ..Default::default()
        }
    }

    pub fn from_audit_args(
        path: PathBuf,
        full: bool,
        analyzers: Option<Vec<String>>,
        config: Option<PathBuf>,
        output_dir: PathBuf,
        formats: Vec<String>,
        severity: String,
        verbose: bool,
        quiet: bool,
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

        Self {
            path,
            analyzers,
            output_formats: formats,
            output_dir,
            severity_threshold,
            config_file: config,
            verbose,
            quiet,
            ..Default::default()
        }
    }

    pub fn should_include_issue(&self, severity: &Severity) -> bool {
        match (&self.severity_threshold, severity) {
            (Severity::Error, Severity::Error) => true,
            (Severity::Warning, Severity::Error | Severity::Warning) => true,
            (Severity::Info, _) => true,
            _ => false,
        }
    }
    
    pub fn from_search_args(
        path: PathBuf,
        keyword: String,
        file_type: String,
        file_pattern: Option<String>,
        case_sensitive: bool,
        line_numbers: bool,
        context: u32,
        output: String,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            analyzers: vec!["search".to_string()],
            output_formats: vec![output],
            verbose,
            quiet,
            ..Default::default()
        }
    }
}

pub fn parse_severity(s: &str) -> Result<Severity, String> {
    match s.to_lowercase().as_str() {
        "error" => Ok(Severity::Error),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        _ => Err(format!("Invalid severity: {}. Use 'error', 'warning', or 'info'", s)),
    }
}