use super::{Analyzer, AnalysisResult};
use crate::ast::{NgProject, Issue, Severity, ProjectMetrics, Recommendation, Priority};
use async_trait::async_trait;
use anyhow::Result;

pub struct StateAnalyzer;

impl StateAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn analyze_state_management(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        let services_with_state = self.identify_state_services(project);
        
        if services_with_state.len() > 3 && !self.has_ngrx_pattern(project) {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "consider-state-management".to_string(),
                message: format!(
                    "Found {} services that appear to manage state. Consider using NgRx or Akita for centralized state management.",
                    services_with_state.len()
                ),
                file_path: project.root_path.clone(),
                line: None,
                column: None,
            });
        }

        for service_name in &services_with_state {
            if let Some(service) = project.services.iter().find(|s| &s.name == service_name) {
                if !service.name.to_lowercase().contains("state") && 
                   !service.name.to_lowercase().contains("store") {
                    issues.push(Issue {
                        severity: Severity::Warning,
                        rule: "unclear-state-service-naming".to_string(),
                        message: format!(
                            "Service '{}' appears to manage state but naming doesn't reflect this. Consider renaming to include 'State' or 'Store'.",
                            service.name
                        ),
                        file_path: service.file_path.clone(),
                        line: None,
                        column: None,
                    });
                }
            }
        }

        issues
    }

    fn identify_state_services(&self, project: &NgProject) -> Vec<String> {
        let mut state_services = Vec::new();
        
        for service in &project.services {
            let state_indicators = [
                "subject", "behaviorsubject", "replaysubject",
                "observable", "state", "store", "cache"
            ];
            
            let has_state_methods = service.methods.iter().any(|method| {
                let method_name_lower = method.name.to_lowercase();
                method_name_lower.contains("get") || 
                method_name_lower.contains("set") || 
                method_name_lower.contains("update") ||
                method_name_lower.contains("state")
            });

            let service_name_lower = service.name.to_lowercase();
            let has_state_naming = state_indicators.iter().any(|indicator| {
                service_name_lower.contains(indicator)
            });

            if has_state_methods || has_state_naming {
                state_services.push(service.name.clone());
            }
        }

        state_services
    }

    fn has_ngrx_pattern(&self, project: &NgProject) -> bool {
        project.services.iter().any(|service| {
            service.name.to_lowercase().contains("store") ||
            service.name.to_lowercase().contains("effect") ||
            service.name.to_lowercase().contains("reducer")
        })
    }

    fn analyze_reactive_patterns(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        for component in &project.components {
            if !component.lifecycle_hooks.contains(&"ngOnDestroy".to_string()) {
                let has_subscription_risk = component.dependencies.iter().any(|dep| {
                    dep.to_lowercase().contains("service") || 
                    dep.to_lowercase().contains("http")
                });

                if has_subscription_risk {
                    issues.push(Issue {
                        severity: Severity::Warning,
                        rule: "missing-unsubscribe-pattern".to_string(),
                        message: format!(
                            "Component '{}' uses services but doesn't implement ngOnDestroy. This may lead to memory leaks from unsubscribed observables.",
                            component.name
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

    fn analyze_change_detection_impact(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();

        let default_cd_components: Vec<_> = project.components.iter()
            .filter(|c| matches!(c.change_detection, crate::ast::ChangeDetectionStrategy::Default))
            .collect();

        let state_heavy_components: Vec<_> = default_cd_components.iter()
            .filter(|c| {
                c.dependencies.iter().any(|dep| {
                    let dep_lower = dep.to_lowercase();
                    dep_lower.contains("state") || 
                    dep_lower.contains("store") || 
                    dep_lower.contains("service")
                })
            })
            .collect();

        if state_heavy_components.len() > 2 {
            issues.push(Issue {
                severity: Severity::Warning,
                rule: "state-change-detection-mismatch".to_string(),
                message: format!(
                    "{} components use state services but have default change detection. Consider OnPush strategy for better performance.",
                    state_heavy_components.len()
                ),
                file_path: project.root_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }

    fn generate_state_recommendations(&self, project: &NgProject) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let state_services = self.identify_state_services(project);
        let has_ngrx = self.has_ngrx_pattern(project);

        if state_services.len() > 1 && !has_ngrx {
            recommendations.push(Recommendation {
                category: "State Management".to_string(),
                title: "Centralize State Management".to_string(),
                description: format!(
                    "Consider implementing NgRx or Akita to centralize state management across {} state services.",
                    state_services.len()
                ),
                priority: Priority::Medium,
                file_path: None,
            });
        }

        let components_without_onpush = project.components.iter()
            .filter(|c| matches!(c.change_detection, crate::ast::ChangeDetectionStrategy::Default))
            .count();

        if components_without_onpush > 0 && !state_services.is_empty() {
            recommendations.push(Recommendation {
                category: "Performance".to_string(),
                title: "Optimize Change Detection".to_string(),
                description: format!(
                    "Implement OnPush change detection strategy in {} components that interact with state services.",
                    components_without_onpush
                ),
                priority: Priority::High,
                file_path: None,
            });
        }

        let components_without_ondestroy = project.components.iter()
            .filter(|c| !c.lifecycle_hooks.contains(&"ngOnDestroy".to_string()))
            .count();

        if components_without_ondestroy > 0 {
            recommendations.push(Recommendation {
                category: "Memory Management".to_string(),
                title: "Implement Proper Cleanup".to_string(),
                description: format!(
                    "Implement ngOnDestroy in {} components to prevent memory leaks from observables.",
                    components_without_ondestroy
                ),
                priority: Priority::High,
                file_path: None,
            });
        }

        recommendations
    }
}

#[async_trait]
impl Analyzer for StateAnalyzer {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult> {
        let mut all_issues = Vec::new();

        all_issues.extend(self.analyze_state_management(project));
        all_issues.extend(self.analyze_reactive_patterns(project));
        all_issues.extend(self.analyze_change_detection_impact(project));

        let recommendations = self.generate_state_recommendations(project);

        Ok(AnalysisResult {
            project: project.clone(),
            issues: all_issues,
            metrics: ProjectMetrics::default(),
            recommendations,
        })
    }

    fn name(&self) -> &'static str {
        "state"
    }

    fn description(&self) -> &'static str {
        "Analyzes state management patterns, reactive programming, and change detection strategies"
    }
}