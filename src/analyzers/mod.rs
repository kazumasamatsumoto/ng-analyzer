use crate::ast::{AnalysisResult, NgProject, Issue, Severity};
use async_trait::async_trait;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;

pub mod component;
pub mod dependency;
pub mod state;
pub mod performance;

#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

#[async_trait]
pub trait Rule: Send + Sync {
    async fn check(&self, node: &AstNode) -> Result<Vec<Issue>>;
    fn severity(&self) -> Severity;
    fn name(&self) -> &'static str;
}

pub struct AstNode {
    pub node_type: String,
    pub content: String,
    pub line: u32,
    pub column: u32,
}

pub struct AnalysisEngine {
    analyzers: HashMap<String, Box<dyn Analyzer>>,
}

impl AnalysisEngine {
    pub fn new() -> Self {
        let mut analyzers: HashMap<String, Box<dyn Analyzer>> = HashMap::new();
        
        analyzers.insert("component".to_string(), Box::new(component::ComponentAnalyzer::new()));
        analyzers.insert("dependency".to_string(), Box::new(dependency::DependencyAnalyzer::new()));
        analyzers.insert("state".to_string(), Box::new(state::StateAnalyzer::new()));
        analyzers.insert("performance".to_string(), Box::new(performance::PerformanceAnalyzer::new()));
        
        Self { analyzers }
    }

    pub async fn run_analysis(&self, project: &NgProject, analyzer_names: &[String]) -> Result<Vec<AnalysisResult>> {
        let results: Result<Vec<_>, _> = analyzer_names
            .par_iter()
            .map(|name| {
                let analyzer = self.analyzers.get(name)
                    .ok_or_else(|| anyhow::anyhow!("Unknown analyzer: {}", name))?;
                
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(analyzer.analyze(project))
                })
            })
            .collect();

        results
    }

    pub fn list_analyzers(&self) -> Vec<&str> {
        self.analyzers.keys().map(|s| s.as_str()).collect()
    }
}