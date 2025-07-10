use super::OutputFormatter;
use crate::ast::AnalysisResult;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tabled::{Table, Tabled};

pub struct TableFormatter {
    show_recommendations: bool,
    show_metrics: bool,
}

impl TableFormatter {
    pub fn new() -> Self {
        Self {
            show_recommendations: true,
            show_metrics: true,
        }
    }

    pub fn new_minimal() -> Self {
        Self {
            show_recommendations: false,
            show_metrics: false,
        }
    }
}

#[derive(Tabled)]
struct IssueRow {
    severity: String,
    rule: String,
    message: String,
    file: String,
    line: String,
}

#[derive(Tabled)]
struct MetricRow {
    metric: String,
    value: String,
}

#[derive(Tabled)]
struct RecommendationRow {
    category: String,
    title: String,
    priority: String,
    description: String,
}

impl OutputFormatter for TableFormatter {
    fn format(&self, results: &[AnalysisResult]) -> Result<String> {
        let mut output = String::new();

        for (i, result) in results.iter().enumerate() {
            if i > 0 {
                output.push_str("\n\n");
            }

            output.push_str(&format!("=== Analysis Result {} ===\n", i + 1));
            output.push_str(&format!("Project: {}\n\n", result.project.root_path.display()));

            if !result.issues.is_empty() {
                output.push_str("Issues:\n");
                let issue_rows: Vec<IssueRow> = result.issues.iter().map(|issue| {
                    IssueRow {
                        severity: format!("{:?}", issue.severity),
                        rule: issue.rule.clone(),
                        message: if issue.message.len() > 80 {
                            format!("{}...", &issue.message[..77])
                        } else {
                            issue.message.clone()
                        },
                        file: std::path::Path::new(&issue.file_path).file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(&issue.file_path)
                            .to_string(),
                        line: issue.line.map(|l| l.to_string()).unwrap_or_else(|| "-".to_string()),
                    }
                }).collect();

                let issues_table = Table::new(issue_rows).to_string();
                output.push_str(&issues_table);
                output.push('\n');
            }

            if self.show_metrics {
                output.push_str("\nMetrics:\n");
                let metric_rows = vec![
                    MetricRow {
                        metric: "Total Components".to_string(),
                        value: result.metrics.total_components.to_string(),
                    },
                    MetricRow {
                        metric: "Total Services".to_string(),
                        value: result.metrics.total_services.to_string(),
                    },
                    MetricRow {
                        metric: "Total Modules".to_string(),
                        value: result.metrics.total_modules.to_string(),
                    },
                    MetricRow {
                        metric: "Average Complexity".to_string(),
                        value: format!("{:.2}", result.metrics.average_complexity),
                    },
                ];

                let metrics_table = Table::new(metric_rows).to_string();
                output.push_str(&metrics_table);
                output.push('\n');
            }

            if self.show_recommendations && !result.recommendations.is_empty() {
                output.push_str("\nRecommendations:\n");
                let recommendation_rows: Vec<RecommendationRow> = result.recommendations.iter().map(|rec| {
                    RecommendationRow {
                        category: rec.category.clone(),
                        title: rec.title.clone(),
                        priority: format!("{:?}", rec.priority),
                        description: if rec.description.len() > 100 {
                            format!("{}...", &rec.description[..97])
                        } else {
                            rec.description.clone()
                        },
                    }
                }).collect();

                let recommendations_table = Table::new(recommendation_rows).to_string();
                output.push_str(&recommendations_table);
                output.push('\n');
            }
        }

        Ok(output)
    }

    fn write_to_file(&self, results: &[AnalysisResult], path: &PathBuf) -> Result<()> {
        let content = self.format(results)?;
        fs::write(path, content)?;
        Ok(())
    }
}