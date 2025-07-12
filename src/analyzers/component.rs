use super::{Analyzer, AnalysisResult};
use crate::ast::{NgProject, NgComponent, Issue, Severity, ChangeDetectionStrategy, ProjectMetrics, Recommendation, Priority};
use async_trait::async_trait;
use anyhow::Result;
use rayon::prelude::*;

pub struct ComponentAnalyzer {
    max_complexity: u32,
    #[allow(dead_code)]
    max_depth: u32,
    max_inputs: usize,
    max_outputs: usize,
}

impl ComponentAnalyzer {
    pub fn new() -> Self {
        Self {
            max_complexity: 10,
            max_depth: 5,
            max_inputs: 10,
            max_outputs: 10,
        }
    }

    #[allow(dead_code)]
    pub fn with_config(max_complexity: u32, max_depth: u32, max_inputs: usize, max_outputs: usize) -> Self {
        Self {
            max_complexity,
            max_depth,
            max_inputs,
            max_outputs,
        }
    }

    fn analyze_component(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        issues.extend(self.check_complexity(component));
        issues.extend(self.check_change_detection(component));
        issues.extend(self.check_inputs_outputs(component));
        issues.extend(self.check_lifecycle_hooks(component));
        issues.extend(self.check_template_style(component));

        issues
    }

    fn check_complexity(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        if component.complexity_score > self.max_complexity {
            issues.push(Issue {
                severity: Severity::Warning,
                rule: "component-complexity".to_string(),
                message: format!(
                    "Component complexity ({}) exceeds threshold ({}). Consider breaking down into smaller components.",
                    component.complexity_score, self.max_complexity
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        if component.complexity_score > self.max_complexity * 2 {
            issues.push(Issue {
                severity: Severity::Error,
                rule: "component-complexity-critical".to_string(),
                message: format!(
                    "Component complexity ({}) is critically high. Immediate refactoring required.",
                    component.complexity_score
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn check_change_detection(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        if matches!(component.change_detection, ChangeDetectionStrategy::Default) {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "change-detection-strategy".to_string(),
                message: "Consider using OnPush change detection strategy for better performance".to_string(),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn check_inputs_outputs(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        if component.inputs.len() > self.max_inputs {
            issues.push(Issue {
                severity: Severity::Warning,
                rule: "too-many-inputs".to_string(),
                message: format!(
                    "Component has {} inputs, which exceeds the recommended maximum of {}",
                    component.inputs.len(), self.max_inputs
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        if component.outputs.len() > self.max_outputs {
            issues.push(Issue {
                severity: Severity::Warning,
                rule: "too-many-outputs".to_string(),
                message: format!(
                    "Component has {} outputs, which exceeds the recommended maximum of {}",
                    component.outputs.len(), self.max_outputs
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn check_lifecycle_hooks(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();
        let hooks = &component.lifecycle_hooks;

        if hooks.contains(&"ngOnInit".to_string()) && hooks.contains(&"ngOnDestroy".to_string()) {
            if !self.has_proper_cleanup_pattern(component) {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "missing-cleanup-pattern".to_string(),
                    message: "Component implements ngOnInit and ngOnDestroy but may be missing proper cleanup patterns (unsubscribe, etc.)".to_string(),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        if hooks.len() > 4 {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "many-lifecycle-hooks".to_string(),
                message: format!(
                    "Component implements {} lifecycle hooks. Consider if all are necessary",
                    hooks.len()
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn check_template_style(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        if component.template.is_some() && component.template_url.is_some() {
            issues.push(Issue {
                severity: Severity::Error,
                rule: "template-conflict".to_string(),
                message: "Component has both inline template and templateUrl. Use only one.".to_string(),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        if component.template.is_none() && component.template_url.is_none() {
            issues.push(Issue {
                severity: Severity::Error,
                rule: "missing-template".to_string(),
                message: "Component must have either a template or templateUrl".to_string(),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        if let Some(template) = &component.template {
            if template.len() > 500 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "inline-template-too-large".to_string(),
                    message: "Inline template is large. Consider using templateUrl instead".to_string(),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn has_proper_cleanup_pattern(&self, _component: &NgComponent) -> bool {
        true
    }

    fn generate_recommendations(&self, project: &NgProject) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let components_with_default_cd: Vec<_> = project.components.iter()
            .filter(|c| matches!(c.change_detection, ChangeDetectionStrategy::Default))
            .collect();

        if !components_with_default_cd.is_empty() {
            recommendations.push(Recommendation {
                category: "Performance".to_string(),
                title: "Optimize Change Detection".to_string(),
                description: format!(
                    "Consider implementing OnPush change detection strategy for {} components to improve performance",
                    components_with_default_cd.len()
                ),
                priority: Priority::Medium,
                file_path: None,
            });
        }

        let high_complexity_components: Vec<_> = project.components.iter()
            .filter(|c| c.complexity_score > self.max_complexity)
            .collect();

        if !high_complexity_components.is_empty() {
            recommendations.push(Recommendation {
                category: "Code Quality".to_string(),
                title: "Reduce Component Complexity".to_string(),
                description: format!(
                    "Break down {} complex components into smaller, more manageable pieces",
                    high_complexity_components.len()
                ),
                priority: Priority::High,
                file_path: None,
            });
        }

        recommendations
    }

    fn calculate_metrics(&self, project: &NgProject) -> ProjectMetrics {
        let total_components = project.components.len() as u32;
        let average_complexity = if total_components > 0 {
            project.components.iter()
                .map(|c| c.complexity_score as f64)
                .sum::<f64>() / total_components as f64
        } else {
            0.0
        };

        ProjectMetrics {
            total_components,
            total_services: project.services.len() as u32,
            total_modules: project.modules.len() as u32,
            average_complexity,
            lines_of_code: 0,
            test_coverage: None,
        }
    }
}

#[async_trait]
impl Analyzer for ComponentAnalyzer {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult> {
        let issues: Vec<Issue> = project.components
            .par_iter()
            .flat_map(|component| self.analyze_component(component))
            .collect();

        let metrics = self.calculate_metrics(project);
        let recommendations = self.generate_recommendations(project);

        Ok(AnalysisResult {
            project: project.clone(),
            issues,
            metrics,
            recommendations,
        })
    }

    fn name(&self) -> &'static str {
        "component"
    }

    fn description(&self) -> &'static str {
        "Analyzes Angular components for best practices, complexity, and performance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_component_analysis() {
        let analyzer = ComponentAnalyzer::new();
        
        let component = NgComponent {
            name: "TestComponent".to_string(),
            file_path: PathBuf::from("test.component.ts"),
            selector: Some("app-test".to_string()),
            template_url: Some("test.component.html".to_string()),
            template: None,
            style_urls: vec!["test.component.css".to_string()],
            inputs: vec![],
            outputs: vec![],
            lifecycle_hooks: vec!["ngOnInit".to_string()],
            dependencies: vec![],
            change_detection: ChangeDetectionStrategy::Default,
            complexity_score: 5,
        };

        let project = NgProject {
            root_path: PathBuf::from("."),
            components: vec![component],
            services: vec![],
            modules: vec![],
            pipes: vec![],
            directives: vec![],
        };

        let result = analyzer.analyze(&project).await.unwrap();
        
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.issues[0].rule, "change-detection-strategy");
        assert_eq!(result.metrics.total_components, 1);
    }

    #[test]
    fn test_complexity_check() {
        let analyzer = ComponentAnalyzer::new();
        
        let component = NgComponent {
            name: "ComplexComponent".to_string(),
            file_path: PathBuf::from("complex.component.ts"),
            selector: Some("app-complex".to_string()),
            template_url: Some("complex.component.html".to_string()),
            template: None,
            style_urls: vec![],
            inputs: vec![],
            outputs: vec![],
            lifecycle_hooks: vec![],
            dependencies: vec![],
            change_detection: ChangeDetectionStrategy::Default,
            complexity_score: 15,
        };

        let issues = analyzer.analyze_component(&component);
        
        let complexity_issues: Vec<_> = issues.iter()
            .filter(|issue| issue.rule.contains("complexity"))
            .collect();
        
        assert!(!complexity_issues.is_empty());
    }
}