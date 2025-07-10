use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgProject {
    pub root_path: PathBuf,
    pub components: Vec<NgComponent>,
    pub services: Vec<NgService>,
    pub modules: Vec<NgModule>,
    pub pipes: Vec<NgPipe>,
    pub directives: Vec<NgDirective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgComponent {
    pub name: String,
    pub file_path: PathBuf,
    pub selector: Option<String>,
    pub template_url: Option<String>,
    pub template: Option<String>,
    pub style_urls: Vec<String>,
    pub inputs: Vec<NgInput>,
    pub outputs: Vec<NgOutput>,
    pub lifecycle_hooks: Vec<String>,
    pub dependencies: Vec<String>,
    pub change_detection: ChangeDetectionStrategy,
    pub complexity_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgService {
    pub name: String,
    pub file_path: PathBuf,
    pub provided_in: Option<String>,
    pub injectable: bool,
    pub dependencies: Vec<String>,
    pub methods: Vec<NgMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgModule {
    pub name: String,
    pub file_path: PathBuf,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub declarations: Vec<String>,
    pub providers: Vec<String>,
    pub bootstrap: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgPipe {
    pub name: String,
    pub file_path: PathBuf,
    pub pure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgDirective {
    pub name: String,
    pub file_path: PathBuf,
    pub selector: String,
    pub inputs: Vec<NgInput>,
    pub outputs: Vec<NgOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgInput {
    pub name: String,
    pub alias: Option<String>,
    pub input_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgOutput {
    pub name: String,
    pub alias: Option<String>,
    pub output_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub complexity_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeDetectionStrategy {
    Default,
    OnPush,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub project: NgProject,
    pub issues: Vec<Issue>,
    pub metrics: ProjectMetrics,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub rule: String,
    pub message: String,
    pub file_path: PathBuf,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMetrics {
    pub total_components: u32,
    pub total_services: u32,
    pub total_modules: u32,
    pub average_complexity: f64,
    pub lines_of_code: u32,
    pub test_coverage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Default for NgProject {
    fn default() -> Self {
        Self {
            root_path: PathBuf::new(),
            components: Vec::new(),
            services: Vec::new(),
            modules: Vec::new(),
            pipes: Vec::new(),
            directives: Vec::new(),
        }
    }
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            project: NgProject::default(),
            issues: Vec::new(),
            metrics: ProjectMetrics::default(),
            recommendations: Vec::new(),
        }
    }
}