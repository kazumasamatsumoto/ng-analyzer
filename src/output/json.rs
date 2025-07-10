use super::OutputFormatter;
use crate::ast::AnalysisResult;
use anyhow::Result;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self { pretty: true }
    }

    pub fn new_compact() -> Self {
        Self { pretty: false }
    }
}

impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &[AnalysisResult]) -> Result<String> {
        let output = if self.pretty {
            serde_json::to_string_pretty(results)?
        } else {
            serde_json::to_string(results)?
        };
        
        Ok(output)
    }

    fn write_to_file(&self, results: &[AnalysisResult], path: &PathBuf) -> Result<()> {
        let content = self.format(results)?;
        fs::write(path, content)?;
        Ok(())
    }
}