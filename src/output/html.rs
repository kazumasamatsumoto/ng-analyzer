use super::OutputFormatter;
use crate::ast::{AnalysisResult, Severity};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct HtmlFormatter {
    include_css: bool,
    include_js: bool,
}

impl HtmlFormatter {
    pub fn new() -> Self {
        Self {
            include_css: true,
            include_js: true,
        }
    }

    pub fn new_minimal() -> Self {
        Self {
            include_css: false,
            include_js: false,
        }
    }

    fn generate_css(&self) -> &'static str {
        r#"
        <style>
            body {
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                line-height: 1.6;
                color: #333;
                max-width: 1200px;
                margin: 0 auto;
                padding: 20px;
                background-color: #f5f5f5;
            }
            
            .header {
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: white;
                padding: 30px;
                border-radius: 10px;
                margin-bottom: 30px;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            }
            
            .header h1 {
                margin: 0;
                font-size: 2.5rem;
                font-weight: 300;
            }
            
            .header .subtitle {
                opacity: 0.9;
                margin-top: 10px;
            }
            
            .analysis-section {
                background: white;
                margin-bottom: 30px;
                border-radius: 10px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                overflow: hidden;
            }
            
            .section-header {
                background: #f8f9fa;
                padding: 20px;
                border-bottom: 1px solid #e9ecef;
            }
            
            .section-header h2 {
                margin: 0;
                color: #495057;
                font-size: 1.5rem;
            }
            
            .section-content {
                padding: 20px;
            }
            
            .issues-grid {
                display: grid;
                gap: 15px;
            }
            
            .issue-card {
                border: 1px solid #e9ecef;
                border-radius: 8px;
                padding: 15px;
                border-left: 4px solid #6c757d;
            }
            
            .issue-card.error {
                border-left-color: #dc3545;
                background-color: #fff5f5;
            }
            
            .issue-card.warning {
                border-left-color: #ffc107;
                background-color: #fffbf0;
            }
            
            .issue-card.info {
                border-left-color: #17a2b8;
                background-color: #f0f9ff;
            }
            
            .issue-severity {
                font-size: 0.8rem;
                font-weight: bold;
                text-transform: uppercase;
                padding: 2px 8px;
                border-radius: 4px;
                display: inline-block;
                margin-bottom: 8px;
            }
            
            .severity-error {
                background-color: #dc3545;
                color: white;
            }
            
            .severity-warning {
                background-color: #ffc107;
                color: #212529;
            }
            
            .severity-info {
                background-color: #17a2b8;
                color: white;
            }
            
            .issue-rule {
                font-weight: 600;
                color: #495057;
                margin-bottom: 5px;
            }
            
            .issue-message {
                color: #6c757d;
                margin-bottom: 10px;
            }
            
            .issue-location {
                font-size: 0.9rem;
                color: #868e96;
            }
            
            .metrics-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                gap: 20px;
            }
            
            .metric-card {
                background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
                padding: 20px;
                border-radius: 8px;
                text-align: center;
            }
            
            .metric-value {
                font-size: 2rem;
                font-weight: bold;
                color: #495057;
                margin-bottom: 5px;
            }
            
            .metric-label {
                color: #6c757d;
                font-size: 0.9rem;
            }
            
            .recommendations-grid {
                display: grid;
                gap: 15px;
            }
            
            .recommendation-card {
                border: 1px solid #e9ecef;
                border-radius: 8px;
                padding: 20px;
                border-left: 4px solid #28a745;
            }
            
            .recommendation-card.high {
                border-left-color: #dc3545;
            }
            
            .recommendation-card.medium {
                border-left-color: #ffc107;
            }
            
            .recommendation-card.low {
                border-left-color: #28a745;
            }
            
            .recommendation-title {
                font-weight: bold;
                color: #495057;
                margin-bottom: 10px;
            }
            
            .recommendation-category {
                font-size: 0.8rem;
                color: #868e96;
                text-transform: uppercase;
                margin-bottom: 5px;
            }
            
            .recommendation-priority {
                font-size: 0.8rem;
                font-weight: bold;
                padding: 2px 8px;
                border-radius: 4px;
                display: inline-block;
                margin-bottom: 10px;
            }
            
            .priority-high {
                background-color: #dc3545;
                color: white;
            }
            
            .priority-medium {
                background-color: #ffc107;
                color: #212529;
            }
            
            .priority-low {
                background-color: #28a745;
                color: white;
            }
            
            .no-issues {
                text-align: center;
                padding: 40px;
                color: #28a745;
                font-size: 1.2rem;
            }
            
            .footer {
                text-align: center;
                padding: 20px;
                color: #6c757d;
                font-size: 0.9rem;
            }
        </style>
        "#
    }

    fn generate_js(&self) -> &'static str {
        r#"
        <script>
            document.addEventListener('DOMContentLoaded', function() {
                const cards = document.querySelectorAll('.issue-card, .recommendation-card');
                cards.forEach(card => {
                    card.addEventListener('mouseenter', function() {
                        this.style.transform = 'translateY(-2px)';
                        this.style.boxShadow = '0 4px 8px rgba(0,0,0,0.15)';
                        this.style.transition = 'all 0.2s ease';
                    });
                    
                    card.addEventListener('mouseleave', function() {
                        this.style.transform = 'translateY(0)';
                        this.style.boxShadow = '0 2px 4px rgba(0,0,0,0.1)';
                    });
                });
            });
        </script>
        "#
    }

    fn severity_to_class(&self, severity: &Severity) -> &'static str {
        match severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }

    fn severity_to_css_class(&self, severity: &Severity) -> &'static str {
        match severity {
            Severity::Error => "severity-error",
            Severity::Warning => "severity-warning",
            Severity::Info => "severity-info",
        }
    }

    fn priority_to_class(&self, priority: &crate::ast::Priority) -> &'static str {
        match priority {
            crate::ast::Priority::High => "priority-high",
            crate::ast::Priority::Medium => "priority-medium",
            crate::ast::Priority::Low => "priority-low",
        }
    }

    fn priority_to_card_class(&self, priority: &crate::ast::Priority) -> &'static str {
        match priority {
            crate::ast::Priority::High => "high",
            crate::ast::Priority::Medium => "medium",
            crate::ast::Priority::Low => "low",
        }
    }
}

impl OutputFormatter for HtmlFormatter {
    fn format(&self, results: &[AnalysisResult]) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("    <title>Angular Analysis Report</title>\n");
        
        if self.include_css {
            html.push_str(self.generate_css());
        }
        
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        html.push_str("    <div class=\"header\">\n");
        html.push_str("        <h1>Angular Analysis Report</h1>\n");
        html.push_str("        <div class=\"subtitle\">Generated by ng-analyzer</div>\n");
        html.push_str("    </div>\n");

        for (_i, result) in results.iter().enumerate() {
            html.push_str(&format!("    <div class=\"analysis-section\">\n"));
            html.push_str(&format!("        <div class=\"section-header\">\n"));
            html.push_str(&format!("            <h2>Project: {}</h2>\n", result.project.root_path.display()));
            html.push_str("        </div>\n");

            if !result.issues.is_empty() {
                html.push_str("        <div class=\"section-content\">\n");
                html.push_str("            <h3>Issues</h3>\n");
                html.push_str("            <div class=\"issues-grid\">\n");
                
                for issue in &result.issues {
                    let severity_class = self.severity_to_class(&issue.severity);
                    let severity_css_class = self.severity_to_css_class(&issue.severity);
                    
                    html.push_str(&format!("                <div class=\"issue-card {}\">\n", severity_class));
                    html.push_str(&format!("                    <div class=\"issue-severity {}\">{:?}</div>\n", severity_css_class, issue.severity));
                    html.push_str(&format!("                    <div class=\"issue-rule\">{}</div>\n", issue.rule));
                    html.push_str(&format!("                    <div class=\"issue-message\">{}</div>\n", issue.message));
                    html.push_str(&format!("                    <div class=\"issue-location\">{}{}</div>\n", 
                        std::path::Path::new(&issue.file_path).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| issue.file_path.clone()),
                        issue.line.map(|l| format!(":{}", l)).unwrap_or_else(|| "".to_string())
                    ));
                    html.push_str("                </div>\n");
                }
                
                html.push_str("            </div>\n");
                html.push_str("        </div>\n");
            } else {
                html.push_str("        <div class=\"section-content\">\n");
                html.push_str("            <div class=\"no-issues\">✅ No issues found!</div>\n");
                html.push_str("        </div>\n");
            }

            html.push_str("        <div class=\"section-content\">\n");
            html.push_str("            <h3>Metrics</h3>\n");
            html.push_str("            <div class=\"metrics-grid\">\n");
            
            html.push_str("                <div class=\"metric-card\">\n");
            html.push_str(&format!("                    <div class=\"metric-value\">{}</div>\n", result.metrics.total_components));
            html.push_str("                    <div class=\"metric-label\">Components</div>\n");
            html.push_str("                </div>\n");
            
            html.push_str("                <div class=\"metric-card\">\n");
            html.push_str(&format!("                    <div class=\"metric-value\">{}</div>\n", result.metrics.total_services));
            html.push_str("                    <div class=\"metric-label\">Services</div>\n");
            html.push_str("                </div>\n");
            
            html.push_str("                <div class=\"metric-card\">\n");
            html.push_str(&format!("                    <div class=\"metric-value\">{}</div>\n", result.metrics.total_modules));
            html.push_str("                    <div class=\"metric-label\">Modules</div>\n");
            html.push_str("                </div>\n");
            
            html.push_str("                <div class=\"metric-card\">\n");
            html.push_str(&format!("                    <div class=\"metric-value\">{:.1}</div>\n", result.metrics.average_complexity));
            html.push_str("                    <div class=\"metric-label\">Avg Complexity</div>\n");
            html.push_str("                </div>\n");
            
            html.push_str("            </div>\n");
            html.push_str("        </div>\n");

            if !result.recommendations.is_empty() {
                html.push_str("        <div class=\"section-content\">\n");
                html.push_str("            <h3>Recommendations</h3>\n");
                html.push_str("            <div class=\"recommendations-grid\">\n");
                
                for rec in &result.recommendations {
                    let priority_class = self.priority_to_class(&rec.priority);
                    let priority_card_class = self.priority_to_card_class(&rec.priority);
                    
                    html.push_str(&format!("                <div class=\"recommendation-card {}\">\n", priority_card_class));
                    html.push_str(&format!("                    <div class=\"recommendation-category\">{}</div>\n", rec.category));
                    html.push_str(&format!("                    <div class=\"recommendation-priority {}\">{:?}</div>\n", priority_class, rec.priority));
                    html.push_str(&format!("                    <div class=\"recommendation-title\">{}</div>\n", rec.title));
                    html.push_str(&format!("                    <div>{}</div>\n", rec.description));
                    html.push_str("                </div>\n");
                }
                
                html.push_str("            </div>\n");
                html.push_str("        </div>\n");
            }

            html.push_str("    </div>\n");
        }

        html.push_str("    <div class=\"footer\">\n");
        html.push_str("        <p>Generated by ng-analyzer - A powerful Angular project analyzer built with Rust</p>\n");
        html.push_str("    </div>\n");
        
        if self.include_js {
            html.push_str(self.generate_js());
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn write_to_file(&self, results: &[AnalysisResult], path: &PathBuf) -> Result<()> {
        let content = self.format(results)?;
        fs::write(path, content)?;
        Ok(())
    }
}