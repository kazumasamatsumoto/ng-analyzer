use super::{Analyzer, AnalysisResult};
use crate::ast::{NgProject, Issue, Severity, ProjectMetrics, Recommendation, Priority};
use async_trait::async_trait;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn analyze_circular_dependencies(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut dependency_graph: HashMap<String, Vec<String>> = HashMap::new();

        for component in &project.components {
            dependency_graph.insert(
                component.name.clone(),
                component.dependencies.clone(),
            );
        }

        for service in &project.services {
            dependency_graph.insert(
                service.name.clone(),
                service.dependencies.clone(),
            );
        }

        if let Some(cycles) = self.detect_cycles(&dependency_graph) {
            for cycle in cycles {
                issues.push(Issue {
                    severity: Severity::Error,
                    rule: "circular-dependency".to_string(),
                    message: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                    file_path: project.root_path.display().to_string().replace('\\', "/"),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn detect_cycles(&self, graph: &HashMap<String, Vec<String>>) -> Option<Vec<Vec<String>>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                if let Some(cycle) = self.dfs_cycles(graph, node, &mut visited, &mut rec_stack, &mut Vec::new()) {
                    cycles.push(cycle);
                }
            }
        }

        if cycles.is_empty() {
            None
        } else {
            Some(cycles)
        }
    }

    fn dfs_cycles(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(cycle) = self.dfs_cycles(graph, neighbor, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(neighbor) {
                    let cycle_start = path.iter().position(|x| x == neighbor).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(neighbor.clone());
                    return Some(cycle);
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }

    fn analyze_unused_dependencies(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut all_dependencies = HashSet::new();
        let mut used_dependencies = HashSet::new();

        for component in &project.components {
            for dep in &component.dependencies {
                all_dependencies.insert(dep.clone());
            }
        }

        for service in &project.services {
            for dep in &service.dependencies {
                all_dependencies.insert(dep.clone());
                used_dependencies.insert(dep.clone());
            }
        }

        for component in &project.components {
            used_dependencies.insert(component.name.clone());
        }

        for service in &project.services {
            used_dependencies.insert(service.name.clone());
        }

        for dep in &all_dependencies {
            if !used_dependencies.contains(dep) {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "unused-dependency".to_string(),
                    message: format!("Dependency '{}' appears to be unused", dep),
                    file_path: project.root_path.display().to_string().replace('\\', "/"),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn analyze_dependency_depth(&self, project: &NgProject) -> Vec<Issue> {
        let mut issues = Vec::new();
        let max_depth = 5;

        for component in &project.components {
            let depth = self.calculate_dependency_depth(&component.name, project, &mut HashSet::new());
            if depth > max_depth {
                issues.push(Issue {
                    severity: Severity::Warning,
                    rule: "deep-dependency-chain".to_string(),
                    message: format!(
                        "Component '{}' has dependency depth of {}, which exceeds recommended maximum of {}",
                        component.name, depth, max_depth
                    ),
                    file_path: component.file_path.clone(),
                    line: None,
                    column: None,
                });
            }
        }

        issues
    }

    fn calculate_dependency_depth(&self, name: &str, project: &NgProject, visited: &mut HashSet<String>) -> u32 {
        if visited.contains(name) {
            return 0;
        }

        visited.insert(name.to_string());

        let mut max_depth = 0;

        if let Some(component) = project.components.iter().find(|c| c.name == name) {
            for dep in &component.dependencies {
                let depth = self.calculate_dependency_depth(dep, project, visited);
                max_depth = max_depth.max(depth);
            }
        }

        if let Some(service) = project.services.iter().find(|s| s.name == name) {
            for dep in &service.dependencies {
                let depth = self.calculate_dependency_depth(dep, project, visited);
                max_depth = max_depth.max(depth);
            }
        }

        visited.remove(name);
        max_depth + 1
    }

    fn generate_dependency_recommendations(&self, project: &NgProject) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        let component_count = project.components.len();
        let service_count = project.services.len();

        if service_count == 0 && component_count > 3 {
            recommendations.push(Recommendation {
                category: "Architecture".to_string(),
                title: "Consider Adding Services".to_string(),
                description: "Your project has multiple components but no services. Consider extracting shared logic into services.".to_string(),
                priority: Priority::Medium,
                file_path: None,
            });
        }

        let total_dependencies: usize = project.components.iter()
            .map(|c| c.dependencies.len())
            .sum::<usize>() + project.services.iter()
            .map(|s| s.dependencies.len())
            .sum::<usize>();

        let avg_dependencies = if (component_count + service_count) > 0 {
            total_dependencies as f64 / (component_count + service_count) as f64
        } else {
            0.0
        };

        if avg_dependencies > 5.0 {
            recommendations.push(Recommendation {
                category: "Dependency Management".to_string(),
                title: "High Dependency Coupling".to_string(),
                description: format!(
                    "Average dependency count is {:.1}. Consider reducing coupling between components and services.",
                    avg_dependencies
                ),
                priority: Priority::Medium,
                file_path: None,
            });
        }

        recommendations
    }
}

#[async_trait]
impl Analyzer for DependencyAnalyzer {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult> {
        let mut all_issues = Vec::new();

        all_issues.extend(self.analyze_circular_dependencies(project));
        all_issues.extend(self.analyze_unused_dependencies(project));
        all_issues.extend(self.analyze_dependency_depth(project));

        let recommendations = self.generate_dependency_recommendations(project);

        Ok(AnalysisResult {
            project: project.clone(),
            issues: all_issues,
            metrics: ProjectMetrics::default(),
            recommendations,
        })
    }

    fn name(&self) -> &'static str {
        "dependency"
    }

    fn description(&self) -> &'static str {
        "Analyzes dependency relationships, circular dependencies, and architectural patterns"
    }
}