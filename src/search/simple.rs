use crate::search::SearchMatch;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

pub struct SimpleSearchEngine {
    pub keyword: String,
    pub case_sensitive: bool,
    pub context: u32,
    #[allow(dead_code)]
    pub line_numbers: bool,
}

impl SimpleSearchEngine {
    pub fn new(keyword: String, case_sensitive: bool, line_numbers: bool, context: u32) -> Self {
        Self {
            keyword,
            case_sensitive,
            context,
            line_numbers,
        }
    }

    pub fn search(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            let search_line = if self.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            let search_keyword = if self.case_sensitive {
                self.keyword.clone()
            } else {
                self.keyword.to_lowercase()
            };

            if let Some(start) = search_line.find(&search_keyword) {
                matches.push(SearchMatch {
                    line_number: line_number + 1,
                    line_content: line.to_string(),
                    match_start: start,
                    match_end: start + search_keyword.len(),
                    context_before: Vec::new(),
                    context_after: Vec::new(),
                    match_type: "simple".to_string(),
                });
            }
        }

        Ok(matches)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub matches: Vec<SearchMatch>,
}

impl SearchResult {
    pub fn total_matches(&self) -> usize {
        self.matches.len()
    }
}