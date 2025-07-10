use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use regex::Regex;

pub mod simple;
pub use simple::{SimpleSearchEngine, SearchResult as SimpleSearchResult};

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
    pub search_type: SearchType,
}

#[derive(Debug, Clone)]
pub enum SearchType {
    Simple,
    Regex,
    HtmlClass,
    HtmlText,
    FunctionName,
    Structural(String), // パターン文字列
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
            search_type: SearchType::Simple,
        }
    }

    pub fn with_search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
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
    pub match_type: String,
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
                let matches = self.search_in_content(&content)?;
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

    fn search_in_content(&self, content: &str) -> Result<Vec<SearchMatch>> {
        match &self.config.search_type {
            SearchType::Simple => self.search_simple(content),
            SearchType::Regex => self.search_regex(content),
            SearchType::HtmlClass => self.search_html_class(content),
            SearchType::HtmlText => self.search_html_text(content),
            SearchType::FunctionName => self.search_function_name(content),
            SearchType::Structural(pattern) => self.search_structural(content, pattern),
        }
    }

    fn search_simple(&self, content: &str) -> Result<Vec<SearchMatch>> {
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
                    match_type: "simple".to_string(),
                });
            }
        }

        Ok(matches)
    }

    fn search_regex(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let regex = if self.config.case_sensitive {
            Regex::new(&self.config.keyword)?
        } else {
            Regex::new(&format!("(?i){}", self.config.keyword))?
        };

        for (line_idx, line) in lines.iter().enumerate() {
            if let Some(cap) = regex.find(line) {
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: cap.start(),
                    match_end: cap.end(),
                    context_before,
                    context_after,
                    match_type: "regex".to_string(),
                });
            }
        }

        Ok(matches)
    }

    fn search_html_class(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // HTMLクラス名を検索するための正規表現
        let class_pattern = format!(r#"class\s*=\s*["']([^"']*\b{}\b[^"']*?)["']"#, regex::escape(&self.config.keyword));
        let regex = if self.config.case_sensitive {
            Regex::new(&class_pattern)?
        } else {
            Regex::new(&format!("(?i){}", class_pattern))?
        };

        for (line_idx, line) in lines.iter().enumerate() {
            if let Some(cap) = regex.find(line) {
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: cap.start(),
                    match_end: cap.end(),
                    context_before,
                    context_after,
                    match_type: "html_class".to_string(),
                });
            }
        }

        Ok(matches)
    }

    fn search_html_text(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // HTMLタグ内のテキストを検索するための正規表現
        let text_pattern = format!(r#">[^<]*\b{}\b[^<]*<"#, regex::escape(&self.config.keyword));
        let regex = if self.config.case_sensitive {
            Regex::new(&text_pattern)?
        } else {
            Regex::new(&format!("(?i){}", text_pattern))?
        };

        for (line_idx, line) in lines.iter().enumerate() {
            if let Some(cap) = regex.find(line) {
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: cap.start(),
                    match_end: cap.end(),
                    context_before,
                    context_after,
                    match_type: "html_text".to_string(),
                });
            }
        }

        Ok(matches)
    }

    fn search_function_name(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // TypeScript/JavaScript関数名を検索するための正規表現
        let function_patterns = [
            // function declaration
            format!(r#"function\s+{}\s*\("#, regex::escape(&self.config.keyword)),
            // arrow function
            format!(r#"(const|let|var)\s+{}\s*=\s*\("#, regex::escape(&self.config.keyword)),
            // method definition
            format!(r#"{}\s*\("#, regex::escape(&self.config.keyword)),
            // async function
            format!(r#"async\s+{}\s*\("#, regex::escape(&self.config.keyword)),
        ];

        for pattern in &function_patterns {
            let regex = if self.config.case_sensitive {
                Regex::new(pattern)?
            } else {
                Regex::new(&format!("(?i){}", pattern))?
            };

            for (line_idx, line) in lines.iter().enumerate() {
                if let Some(cap) = regex.find(line) {
                    let context_before = self.get_context_lines(&lines, line_idx, true);
                    let context_after = self.get_context_lines(&lines, line_idx, false);

                    matches.push(SearchMatch {
                        line_number: line_idx + 1,
                        line_content: line.to_string(),
                        match_start: cap.start(),
                        match_end: cap.end(),
                        context_before,
                        context_after,
                        match_type: "function_name".to_string(),
                    });
                }
            }
        }

        Ok(matches)
    }

    fn search_structural(&self, content: &str, pattern: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // 構造的パターンを処理
        let regex = if self.config.case_sensitive {
            Regex::new(pattern)?
        } else {
            Regex::new(&format!("(?i){}", pattern))?
        };

        for (line_idx, line) in lines.iter().enumerate() {
            if let Some(cap) = regex.find(line) {
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: cap.start(),
                    match_end: cap.end(),
                    context_before,
                    context_after,
                    match_type: "structural".to_string(),
                });
            }
        }

        Ok(matches)
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