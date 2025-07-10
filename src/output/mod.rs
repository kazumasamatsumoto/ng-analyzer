pub mod json;
pub mod html;
pub mod table;

use crate::ast::AnalysisResult;
use anyhow::Result;
use std::path::PathBuf;

pub use json::JsonFormatter;
pub use html::HtmlFormatter;
pub use table::TableFormatter;

pub trait OutputFormatter {
    fn format(&self, results: &[AnalysisResult]) -> Result<String>;
    fn write_to_file(&self, results: &[AnalysisResult], path: &PathBuf) -> Result<()>;
}

pub struct MultiFormatter {
    formatters: Vec<(String, Box<dyn OutputFormatter>)>,
}

impl MultiFormatter {
    pub fn new() -> Self {
        Self {
            formatters: Vec::new(),
        }
    }

    pub fn add_formatter(&mut self, name: String, formatter: Box<dyn OutputFormatter>) {
        self.formatters.push((name, formatter));
    }

    pub fn format_all(&self, results: &[AnalysisResult], output_dir: &PathBuf) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;

        for (name, formatter) in &self.formatters {
            let file_path = output_dir.join(format!("analysis-report.{}", name));
            formatter.write_to_file(results, &file_path)?;
        }

        Ok(())
    }
}

pub fn create_formatter(format: &str) -> Result<Box<dyn OutputFormatter>> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Box::new(JsonFormatter::new())),
        "html" => Ok(Box::new(HtmlFormatter::new())),
        "table" => Ok(Box::new(TableFormatter::new())),
        _ => Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }
}

pub fn create_multi_formatter(formats: &[String]) -> Result<MultiFormatter> {
    let mut multi = MultiFormatter::new();
    
    for format in formats {
        let formatter = create_formatter(format)?;
        multi.add_formatter(format.clone(), formatter);
    }
    
    Ok(multi)
}