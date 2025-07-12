use anyhow::Result;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub mod simple;
pub use simple::SimpleSearchEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub path: PathBuf,
    pub keyword: String,
    pub file_type: Option<String>,
    #[allow(dead_code)]
    pub file_pattern: Option<String>,
    pub case_sensitive: bool,
    pub line_numbers: bool,
    pub context: u32,
    pub output_format: String,
    #[allow(dead_code)]
    pub verbose: bool,
}

impl SearchConfig {
    pub fn new(
        path: PathBuf,
        keyword: String,
        file_type: Option<String>,
        file_pattern: Option<String>,
        case_sensitive: bool,
        line_numbers: bool,
        context: u32,
        output_format: String,
        verbose: bool,
    ) -> Self {
        Self {
            path,
            keyword,
            file_type,
            file_pattern,
            case_sensitive,
            line_numbers,
            context,
            output_format,
            verbose,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Simple,
    Regex(String),
    HtmlClass(String),
    HtmlText(String),
    FunctionName(String),
    #[allow(dead_code)]
    Structural(String), // パターン文字列
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: PathBuf,
    pub total_matches: usize,
    pub matches: Vec<SearchMatch>,
    pub search_type: SearchType,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSummary {
    pub total_files_searched: usize,
    pub files_with_matches: usize,
    pub total_matches: usize,
    pub search_config: SearchConfig,
}

#[allow(dead_code)]
pub struct SearchEngine {
    config: SearchConfig,
}

impl SearchEngine {
    #[allow(dead_code)]
    pub fn new(config: SearchConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    pub async fn search(&self) -> Result<Vec<SearchResult>> {
        let files = self.collect_files().await?;
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(content) = tokio::fs::read_to_string(&file_path).await {
                let matches = self.search_in_content(&content)?;
                if !matches.is_empty() {
                    results.push(SearchResult {
                        file_path,
                        total_matches: matches.len(),
                        matches,
                        search_type: SearchType::Simple,
                    });
                }
            }
        }

        Ok(results)
    }

    #[allow(dead_code)]
    async fn collect_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.config.path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() && self.should_include_file(&path) {
                files.push(path);
            } else if path.is_dir() {
                // 再帰的にディレクトリを探索
                let mut subdir = tokio::fs::read_dir(&path).await?;
                while let Some(subentry) = subdir.next_entry().await? {
                    let subpath = subentry.path();
                    if subpath.is_file() && self.should_include_file(&subpath) {
                        files.push(subpath);
                    }
                }
            }
        }

        Ok(files)
    }

    #[allow(dead_code)]
    fn should_include_file(&self, path: &std::path::Path) -> bool {
        if let Some(file_type) = &self.config.file_type {
            if let Some(extension) = path.extension() {
                return extension.to_str() == Some(file_type);
            }
            return false;
        }

        // デフォルトで TypeScript/JavaScript/HTML ファイルを対象とする
        if let Some(extension) = path.extension() {
            matches!(extension.to_str(), Some("ts") | Some("js") | Some("html") | Some("htm"))
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn search_in_content(&self, content: &str) -> Result<Vec<SearchMatch>> {
        match &self.config.keyword {
            _ => self.search_simple(content),
        }
    }

    #[allow(dead_code)]
    fn search_simple(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_number, line) in lines.iter().enumerate() {
            let search_line = if self.config.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            let search_keyword = if self.config.case_sensitive {
                self.config.keyword.clone()
            } else {
                self.config.keyword.to_lowercase()
            };

            if let Some(start) = search_line.find(&search_keyword) {
                let context_before = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, true)
                } else {
                    Vec::new()
                };

                let context_after = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, false)
                } else {
                    Vec::new()
                };

                matches.push(SearchMatch {
                    line_number: line_number + 1,
                    line_content: line.to_string(),
                    match_start: start,
                    match_end: start + search_keyword.len(),
                    context_before,
                    context_after,
                    match_type: "simple".to_string(),
                });
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn search_regex(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let regex = regex::Regex::new(&self.config.keyword)?;

        for (line_number, line) in lines.iter().enumerate() {
            for mat in regex.find_iter(line) {
                let context_before = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, true)
                } else {
                    Vec::new()
                };

                let context_after = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, false)
                } else {
                    Vec::new()
                };

                matches.push(SearchMatch {
                    line_number: line_number + 1,
                    line_content: line.to_string(),
                    match_start: mat.start(),
                    match_end: mat.end(),
                    context_before,
                    context_after,
                    match_type: "regex".to_string(),
                });
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn search_html_class(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let class_regex = regex::Regex::new(r#"class\s*=\s*["']([^"']*)"#)?;

        for (line_number, line) in lines.iter().enumerate() {
            for cap in class_regex.captures_iter(line) {
                if let Some(class_attr) = cap.get(1) {
                    let classes = class_attr.as_str();
                    if classes.contains(&self.config.keyword) {
                        let context_before = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, true)
                        } else {
                            Vec::new()
                        };

                        let context_after = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, false)
                        } else {
                            Vec::new()
                        };

                        matches.push(SearchMatch {
                            line_number: line_number + 1,
                            line_content: line.to_string(),
                            match_start: class_attr.start(),
                            match_end: class_attr.end(),
                            context_before,
                            context_after,
                            match_type: "html_class".to_string(),
                        });
                    }
                }
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn search_html_text(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let text_regex = regex::Regex::new(r#">([^<]*)<"#)?;

        for (line_number, line) in lines.iter().enumerate() {
            for cap in text_regex.captures_iter(line) {
                if let Some(text_content) = cap.get(1) {
                    let text = text_content.as_str().trim();
                    if !text.is_empty() && text.contains(&self.config.keyword) {
                        let context_before = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, true)
                        } else {
                            Vec::new()
                        };

                        let context_after = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, false)
                        } else {
                            Vec::new()
                        };

                        matches.push(SearchMatch {
                            line_number: line_number + 1,
                            line_content: line.to_string(),
                            match_start: text_content.start(),
                            match_end: text_content.end(),
                            context_before,
                            context_after,
                            match_type: "html_text".to_string(),
                        });
                    }
                }
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn search_function_name(&self, content: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let function_regex = regex::Regex::new(r#"(function\s+|async\s+function\s+|^\s*)([\w$]+)\s*\("#)?;

        for (line_number, line) in lines.iter().enumerate() {
            for cap in function_regex.captures_iter(line) {
                if let Some(func_name) = cap.get(2) {
                    if func_name.as_str().contains(&self.config.keyword) {
                        let context_before = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, true)
                        } else {
                            Vec::new()
                        };

                        let context_after = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, false)
                        } else {
                            Vec::new()
                        };

                        matches.push(SearchMatch {
                            line_number: line_number + 1,
                            line_content: line.to_string(),
                            match_start: func_name.start(),
                            match_end: func_name.end(),
                            context_before,
                            context_after,
                            match_type: "function_name".to_string(),
                        });
                    }
                }
            }
        }

        // TypeScript のメソッド定義もチェック
        let method_regex = regex::Regex::new(r#"^\s*(public|private|protected)?\s*(async\s+)?([\w$]+)\s*\("#)?;
        for (line_number, line) in lines.iter().enumerate() {
            for cap in method_regex.captures_iter(line) {
                if let Some(method_name) = cap.get(3) {
                    if method_name.as_str().contains(&self.config.keyword) {
                        let context_before = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, true)
                        } else {
                            Vec::new()
                        };

                        let context_after = if self.config.context > 0 {
                            self.get_context_lines(&lines, line_number, false)
                        } else {
                            Vec::new()
                        };

                        matches.push(SearchMatch {
                            line_number: line_number + 1,
                            line_content: line.to_string(),
                            match_start: method_name.start(),
                            match_end: method_name.end(),
                            context_before,
                            context_after,
                            match_type: "function_name".to_string(),
                        });
                    }
                }
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn search_structural(&self, content: &str, pattern: &str) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let regex = regex::Regex::new(pattern)?;

        for (line_number, line) in lines.iter().enumerate() {
            for mat in regex.find_iter(line) {
                let context_before = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, true)
                } else {
                    Vec::new()
                };

                let context_after = if self.config.context > 0 {
                    self.get_context_lines(&lines, line_number, false)
                } else {
                    Vec::new()
                };

                matches.push(SearchMatch {
                    line_number: line_number + 1,
                    line_content: line.to_string(),
                    match_start: mat.start(),
                    match_end: mat.end(),
                    context_before,
                    context_after,
                    match_type: "structural".to_string(),
                });
            }
        }

        Ok(matches)
    }

    #[allow(dead_code)]
    fn get_context_lines(&self, lines: &[&str], current_line: usize, before: bool) -> Vec<String> {
        let mut context = Vec::new();
        let context_size = self.config.context as usize;

        if before {
            let start = current_line.saturating_sub(context_size);
            for i in start..current_line {
                if i < lines.len() {
                    context.push(lines[i].to_string());
                }
            }
        } else {
            let end = (current_line + context_size + 1).min(lines.len());
            for i in (current_line + 1)..end {
                if i < lines.len() {
                    context.push(lines[i].to_string());
                }
            }
        }

        context
    }
}