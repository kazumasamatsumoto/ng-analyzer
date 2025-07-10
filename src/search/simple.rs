use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;

pub struct SimpleSearchEngine {
    pub keyword: String,
    pub file_type: String,
    pub case_sensitive: bool,
    pub line_numbers: bool,
    pub context: u32,
}

impl SimpleSearchEngine {
    pub fn new(keyword: String, file_type: String, case_sensitive: bool, line_numbers: bool, context: u32) -> Self {
        Self {
            keyword,
            file_type,
            case_sensitive,
            line_numbers,
            context,
        }
    }

    pub fn search_in_directory(&self, path: &PathBuf) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.is_file() && self.should_include_file(entry_path) {
                if let Ok(content) = fs::read_to_string(entry_path) {
                    let matches = self.search_in_content(&content);
                    if !matches.is_empty() {
                        results.push(SearchResult {
                            file_path: entry_path.to_string_lossy().to_string(),
                            matches,
                        });
                    }
                }
            }
        }
        
        results
    }

    fn should_include_file(&self, path: &std::path::Path) -> bool {
        match self.file_type.as_str() {
            "html" => path.extension().map_or(false, |ext| ext == "html" || ext == "htm"),
            "ts" => path.extension().map_or(false, |ext| ext == "ts"),
            "js" => path.extension().map_or(false, |ext| ext == "js"),
            "all" => {
                if let Some(ext) = path.extension() {
                    matches!(ext.to_str(), Some("html") | Some("htm") | Some("ts") | Some("js") | Some("css") | Some("scss") | Some("json") | Some("md") | Some("txt") | Some("rs"))
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

        let search_keyword = if self.case_sensitive {
            self.keyword.clone()
        } else {
            self.keyword.to_lowercase()
        };

        for (line_idx, line) in lines.iter().enumerate() {
            let search_line = if self.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            if search_line.contains(&search_keyword) {
                let context_before = self.get_context_lines(&lines, line_idx, true);
                let context_after = self.get_context_lines(&lines, line_idx, false);

                matches.push(SearchMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    context_before,
                    context_after,
                });
            }
        }

        matches
    }

    fn get_context_lines(&self, lines: &[&str], current_line: usize, before: bool) -> Vec<String> {
        if self.context == 0 {
            return Vec::new();
        }

        let mut context = Vec::new();
        let context_count = self.context as usize;

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

pub struct SearchResult {
    pub file_path: String,
    pub matches: Vec<SearchMatch>,
}

pub struct SearchMatch {
    pub line_number: usize,
    pub line_content: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

impl SearchResult {
    pub fn total_matches(&self) -> usize {
        self.matches.len()
    }
}