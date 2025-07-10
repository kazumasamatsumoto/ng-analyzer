use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod simple;
pub use simple::{SimpleSearchEngine, SearchResult as SimpleSearchResult, SearchMatch as SimpleSearchMatch};

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub path: PathBuf,
    pub keyword: String,
    pub file_type: String,
    pub file_pattern: Option<String>,
    pub case_sensitive: bool,
    pub line_numbers: bool,
    pub context: u32,
    pub output: String,
    pub verbose: bool,
    pub quiet: bool,
}

impl SearchConfig {
    pub fn new(
        path: PathBuf,
        keyword: String,
        file_type: String,
        file_pattern: Option<String>,
        case_sensitive: bool,
        line_numbers: bool,
        context: u32,
        output: String,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            path,
            keyword,
            file_type,
            file_pattern,
            case_sensitive,
            line_numbers,
            context,
            output,
            verbose,
            quiet,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub matches: Vec<SearchMatch>,
    pub total_matches: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub line_number: usize,
    pub line_content: String,
    pub match_start: usize,
    pub match_end: usize,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

pub struct SearchEngine {
    config: SearchConfig,
}

impl SearchEngine {
    pub fn new(config: SearchConfig) -> Self {
        Self { config }
    }

    pub async fn search(&self) -> Result<Vec<SearchResult>> {
        if !self.config.quiet {
            println!("🔍 Searching for '{}' in {}", self.config.keyword, self.config.path.display());
        }

        let mut results = Vec::new();
        let files = self.collect_files().await?;

        for file_path in files {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let matches = self.search_in_content(&content);
                if !matches.is_empty() {
                    results.push(SearchResult {
                        file_path: file_path.to_string_lossy().to_string(),
                        total_matches: matches.len(),
                        matches,
                    });
                }
            }
        }

        Ok(results)
    }

    async fn collect_files(&self) -> Result<Vec<PathBuf>> {
        use walkdir::WalkDir;
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.config.path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && self.should_include_file(path) {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    fn should_include_file(&self, path: &std::path::Path) -> bool {
        // Check file pattern first
        if let Some(pattern) = &self.config.file_pattern {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                return file_name.contains(pattern);
            }
            return false;
        }

        // Check file type
        match self.config.file_type.as_str() {
            "html" => path.extension().map_or(false, |ext| ext == "html" || ext == "htm"),
            "ts" => path.extension().map_or(false, |ext| ext == "ts"),
            "js" => path.extension().map_or(false, |ext| ext == "js"),
            "all" => {
                if let Some(ext) = path.extension() {
                    matches!(ext.to_str(), Some("html") | Some("htm") | Some("ts") | Some("js") | Some("css") | Some("scss") | Some("json") | Some("md"))
                } else {
                    false
                }
            }
            _ => true,
        }
    }

    fn search_in_content(&self, content: &str) -> Vec<SearchMatch> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let search_keyword = if self.config.case_sensitive {
            self.config.keyword.clone()
        } else {
            self.config.keyword.to_lowercase()
        };

        for (line_idx, line) in lines.iter().enumerate() {
            let search_line = if self.config.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            if let Some(match_start) = search_line.find(&search_keyword) {
                let match_end = match_start + search_keyword.len();
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start,
                    match_end,
                    context_before,
                    context_after,
                });
            }
        }

        matches
    }

    fn get_context_lines(&self, lines: &[&str], current_line: usize, before: bool) -> Vec<String> {
        if self.config.context == 0 {
            return Vec::new();
        }

        let mut context = Vec::new();
        let context_count = self.config.context as usize;

        if before {
            let start = current_line.saturating_sub(context_count);
            for i in start..current_line {
                context.push(lines[i].to_string());
            }
        } else {
            let end = std::cmp::min(current_line + context_count + 1, lines.len());
            for i in (current_line + 1)..end {
                context.push(lines[i].to_string());
            }
        }

        context
    }
}