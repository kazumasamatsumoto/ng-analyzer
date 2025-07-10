use super::{Analyzer, AnalysisResult};
use crate::ast::{NgProject, Issue, Severity, ProjectMetrics, Recommendation, Priority};
use async_trait::async_trait;
use anyhow::Result;

pub struct PerformanceAnalyzer;

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn analyze_bundle_size_impact(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        for component in &project.components {
            if component.style_urls.len() > 3 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "too-many-stylesheets".to_string(),
                    message: format!(
                        "Component '{}' has {} stylesheets. Consider consolidating styles.",
                        component.name, component.style_urls.len()
                    ),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }

            if let Some(template) = &component.template {
                if template.len() > 2000 {
                    issues.push(Issue {
                        severity: Severity::Warning,
                        rule: "large-inline-template".to_string(),
                        message: format!(
                            "Component '{}' has a large inline template ({} characters). Consider using templateUrl.",
                            component.name, template.len()
                        ),
                        file_path: component.file_path.clone(),
                        line: None,
                        column: None,
                    });
                }
            }
        }

        issues
    }

    fn analyze_change_detection_performance(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        let default_cd_count = project.components.iter()
            .filter(|c| matches!(c.change_detection, crate::ast::ChangeDetectionStrategy::Default))
            .count();

        let total_components = project.components.len();

        if total_components > 0 {
            let default_percentage = (default_cd_count as f64 / total_components as f64) * 100.0;
            
            if default_percentage > 70.0 && total_components > 5 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "high-default-change-detection".to_string(),
                    message: format!(
                        "{:.1}% of components use default change detection. Consider OnPush for better performance.",
                        default_percentage
                    ),
                    file_path: project.root_path.display().to_string().replace('\\', "/"),
                    line: None,
                    column: None,
                });
            }
        }

        for component in &project.components {
            if matches!(component.change_detection, crate::ast::ChangeDetectionStrategy::Default) 
                && component.complexity_score > 8 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "complex-component-default-cd".to_string(),
                    message: format!(
                        "Complex component '{}' (score: {}) uses default change detection. Consider OnPush strategy.",
                        component.name, component.complexity_score
                    ),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn analyze_lazy_loading_opportunities(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        if project.modules.len() == 1 && project.components.len() > 10 {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "consider-lazy-loading".to_string(),
                message: format!(
                    "Project has {} components in a single module. Consider implementing lazy-loaded feature modules.",
                    project.components.len()
                ),
                file_path: project.root_path.display().to_string(),
                line: None,
                column: None,
            });
        }

        let feature_components_ratio = if project.modules.len() > 1 {
            project.components.len() as f64 / project.modules.len() as f64
        } else {
            project.components.len() as f64
        };

        if feature_components_ratio > 8.0 && project.modules.len() > 1 {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "unbalanced-modules".to_string(),
                message: format!(
                    "Average of {:.1} components per module. Consider better module organization for lazy loading.",
                    feature_components_ratio
                ),
                file_path: project.root_path.display().to_string(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn analyze_memory_leaks_risk(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        for component in &project.components {
            let has_http_dependencies = component.dependencies.iter()
                .any(|dep| dep.to_lowercase().contains("http"));
            
            let has_service_dependencies = component.dependencies.iter()
                .any(|dep| dep.to_lowercase().contains("service"));

            let implements_on_destroy = component.lifecycle_hooks.contains(&"ngOnDestroy".to_string());

            if (has_http_dependencies || has_service_dependencies) && !implements_on_destroy {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "potential-memory-leak".to_string(),
                    message: format!(
                        "Component '{}' uses HTTP/services but doesn't implement ngOnDestroy. Potential memory leak risk.",
                        component.name
                    ),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn analyze_excessive_watchers(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        for component in &project.components {
            let total_bindings = component.inputs.len() + component.outputs.len();
            
            if total_bindings > 15 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "excessive-bindings".to_string(),
                    message: format!(
                        "Component '{}' has {} bindings. Consider reducing to improve change detection performance.",
                        component.name, total_bindings
                    ),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn generate_performance_recommendations(&self, project: &NgProject) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let onpush_candidates = project.components.iter()
            .filter(|c| matches!(c.change_detection, crate::ast::ChangeDetectionStrategy::Default))
            .filter(|c| c.complexity_score > 5 || c.inputs.len() + c.outputs.len() > 5)
            .count();

        if onpush_candidates > 0 {
            recommendations.push(Recommendation {
                category: "Performance".to_string(),
                title: "Implement OnPush Change Detection".to_string(),
                description: format!(
                    "Implement OnPush change detection in {} components to improve performance and reduce unnecessary re-renders.",
                    onpush_candidates
                ),
                priority: Priority::High,
                file_path: None,
            });
        }

        if project.modules.len() == 1 && project.components.len() > 8 {
            recommendations.push(Recommendation {
                category: "Performance".to_string(),
                title: "Implement Lazy Loading".to_string(),
                description: "Split your application into feature modules with lazy loading to reduce initial bundle size and improve startup performance.".to_string(),
                priority: Priority::Medium,
                file_path: None,
            });
        }

        let components_with_memory_risk = project.components.iter()
            .filter(|c| {
                let has_services = c.dependencies.iter().any(|dep| 
                    dep.to_lowercase().contains("service") || dep.to_lowercase().contains("http"));
                let no_ondestroy = !c.lifecycle_hooks.contains(&"ngOnDestroy".to_string());
                has_services && no_ondestroy
            })
            .count();

        if components_with_memory_risk > 0 {
            recommendations.push(Recommendation {
                category: "Memory Management".to_string(),
                title: "Prevent Memory Leaks".to_string(),
                description: format!(
                    "Implement proper cleanup patterns in {} components to prevent memory leaks from observables and event listeners.",
                    components_with_memory_risk
                ),
                priority: Priority::High,
                file_path: None,
            });
        }

        let inline_template_components = project.components.iter()
            .filter(|c| c.template.as_ref().map_or(false, |t| t.len() > 500))
            .count();

        if inline_template_components > 0 {
            recommendations.push(Recommendation {
                category: "Bundle Size".to_string(),
                title: "Optimize Template Size".to_string(),
                description: format!(
                    "Move {} large inline templates to external files to improve build performance and enable template caching.",
                    inline_template_components
                ),
                priority: Priority::Low,
                file_path: None,
            });
        }

        recommendations
    }

    fn calculate_performance_metrics(&self, project: &NgProject) -> ProjectMetrics {
        let total_components = project.components.len() as u32;
        let onpush_components = project.components.iter()
            .filter(|c| matches!(c.change_detection, crate::ast::ChangeDetectionStrategy::OnPush))
            .count() as u32;

        let onpush_percentage = if total_components > 0 {
            (onpush_components as f64 / total_components as f64) * 100.0
        } else {
            0.0
        };

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
            test_coverage: Some(onpush_percentage),
        }
    }
}

#[async_trait]
impl Analyzer for PerformanceAnalyzer {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult> {
        let mut all_issues = Vec::new();

        all_issues.extend(self.analyze_bundle_size_impact(project));
        all_issues.extend(self.analyze_change_detection_performance(project));
        all_issues.extend(self.analyze_lazy_loading_opportunities(project));
        all_issues.extend(self.analyze_memory_leaks_risk(project));
        all_issues.extend(self.analyze_excessive_watchers(project));

        let recommendations = self.generate_performance_recommendations(project);
        let metrics = self.calculate_performance_metrics(project);

        Ok(AnalysisResult {
            project: project.clone(),
            issues: all_issues,
            metrics,
            recommendations,
        })
    }

    fn name(&self) -> &'static str {
        "performance"
    }

    fn description(&self) -> &'static str {
        "Analyzes performance implications, change detection strategies, and optimization opportunities"
    }
}