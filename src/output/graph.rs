use crate::ast::{ImportExportGraph, DependencyAnalysis};
use anyhow::Result;
use std::path::Path;

pub struct GraphFormatter;

impl GraphFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_dot(&self, graph: &ImportExportGraph, analysis: &DependencyAnalysis) -> Result<String> {
        let mut output = String::new();
        
        output.push_str("digraph dependency_graph {\n");
        output.push_str("    rankdir=TB;\n");
        output.push_str("    node [shape=rectangle, style=filled];\n");
        output.push_str("    edge [fontsize=10];\n\n");
        
        // ノード（ファイル）を定義
        for file in &graph.files {
            let node_id = self.sanitize_node_id(&file.id);
            let file_name = Path::new(&file.file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&file.relative_path);
            
            // ファイルタイプによって色を変更
            let color = match file.file_type {
                crate::ast::FileType::TypeScript => "lightblue",
                crate::ast::FileType::JavaScript => "lightgreen",
                crate::ast::FileType::Declaration => "lightyellow",
                crate::ast::FileType::Module => "lightgray",
            };
            
            output.push_str(&format!(
                "    {} [label=\"{}\", fillcolor={}, tooltip=\"{}\"];\n",
                node_id, file_name, color, file.relative_path
            ));
        }
        
        output.push_str("\n");
        
        // エッジ（依存関係）を定義
        for dependency in &graph.dependencies {
            let from_node = self.sanitize_node_id(&dependency.from_file);
            let to_node = self.sanitize_node_id(&dependency.to_file);
            
            let label = if dependency.imported_symbols.len() > 3 {
                format!("{}個のシンボル", dependency.imported_symbols.len())
            } else {
                dependency.imported_symbols.join(", ")
            };
            
            let color = match dependency.import_type {
                crate::ast::ImportType::Default => "blue",
                crate::ast::ImportType::Named => "green",
                crate::ast::ImportType::Namespace => "orange",
                crate::ast::ImportType::Dynamic => "red",
            };
            
            output.push_str(&format!(
                "    {} -> {} [label=\"{}\", color={}, tooltip=\"{}\"];\n",
                from_node, to_node, label, color, 
                format!("Type: {:?}", dependency.import_type)
            ));
        }
        
        // 循環依存を強調
        if !analysis.circular_dependencies.is_empty() {
            output.push_str("\n    // 循環依存\n");
            for circular in &analysis.circular_dependencies {
                for i in 0..circular.cycle.len() - 1 {
                    let from_node = self.sanitize_node_id(&circular.cycle[i]);
                    let to_node = self.sanitize_node_id(&circular.cycle[i + 1]);
                    output.push_str(&format!(
                        "    {} -> {} [color=red, style=bold, penwidth=2];\n",
                        from_node, to_node
                    ));
                }
            }
        }
        
        output.push_str("}\n");
        
        Ok(output)
    }

    pub fn format_mermaid(&self, graph: &ImportExportGraph, analysis: &DependencyAnalysis) -> Result<String> {
        let mut output = String::new();
        
        output.push_str("graph TD\n");
        
        // ノード（ファイル）を定義
        for file in &graph.files {
            let node_id = self.sanitize_node_id(&file.id);
            let file_name = Path::new(&file.file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&file.relative_path);
            
            // ファイルタイプによってスタイルを変更
            let style = match file.file_type {
                crate::ast::FileType::TypeScript => "fill:#e1f5fe,stroke:#01579b",
                crate::ast::FileType::JavaScript => "fill:#e8f5e8,stroke:#2e7d32",
                crate::ast::FileType::Declaration => "fill:#fff3e0,stroke:#e65100",
                crate::ast::FileType::Module => "fill:#f5f5f5,stroke:#424242",
            };
            
            output.push_str(&format!(
                "    {}[\"{}\"]\n",
                node_id, file_name
            ));
            
            output.push_str(&format!(
                "    style {} {}\n",
                node_id, style
            ));
        }
        
        output.push_str("\n");
        
        // エッジ（依存関係）を定義
        for dependency in &graph.dependencies {
            let from_node = self.sanitize_node_id(&dependency.from_file);
            let to_node = self.sanitize_node_id(&dependency.to_file);
            
            let label = if dependency.imported_symbols.len() > 3 {
                format!("{}個", dependency.imported_symbols.len())
            } else {
                dependency.imported_symbols.join(",")
            };
            
            output.push_str(&format!(
                "    {} -->|{}| {}\n",
                from_node, label, to_node
            ));
        }
        
        // 循環依存を強調
        if !analysis.circular_dependencies.is_empty() {
            output.push_str("\n    %% 循環依存\n");
            for circular in &analysis.circular_dependencies {
                for i in 0..circular.cycle.len() - 1 {
                    let from_node = self.sanitize_node_id(&circular.cycle[i]);
                    let to_node = self.sanitize_node_id(&circular.cycle[i + 1]);
                    output.push_str(&format!(
                        "    {} -.->|循環| {}\n",
                        from_node, to_node
                    ));
                    output.push_str(&format!(
                        "    linkStyle {} stroke:#ff0000,stroke-width:3px\n",
                        i
                    ));
                }
            }
        }
        
        Ok(output)
    }

    pub fn format_json(&self, graph: &ImportExportGraph, analysis: &DependencyAnalysis) -> Result<String> {
        let combined_output = serde_json::json!({
            "graph": graph,
            "analysis": analysis,
            "summary": {
                "total_files": graph.files.len(),
                "total_dependencies": graph.dependencies.len(),
                "circular_dependencies": analysis.circular_dependencies.len(),
                "orphaned_files": analysis.orphaned_files.len()
            }
        });
        
        Ok(serde_json::to_string_pretty(&combined_output)?)
    }

    pub fn format_table(&self, graph: &ImportExportGraph, analysis: &DependencyAnalysis) -> Result<String> {
        let mut output = String::new();
        
        output.push_str("# 依存関係グラフ分析結果\n\n");
        
        // サマリー
        output.push_str("## サマリー\n");
        output.push_str(&format!("- 総ファイル数: {}\n", graph.files.len()));
        output.push_str(&format!("- 総依存関係数: {}\n", graph.dependencies.len()));
        output.push_str(&format!("- 循環依存数: {}\n", analysis.circular_dependencies.len()));
        output.push_str(&format!("- 孤立ファイル数: {}\n", analysis.orphaned_files.len()));
        output.push_str("\n");
        
        // 循環依存
        if !analysis.circular_dependencies.is_empty() {
            output.push_str("## 循環依存\n");
            for (i, circular) in analysis.circular_dependencies.iter().enumerate() {
                output.push_str(&format!(
                    "{}. {} (重要度: {:?})\n",
                    i + 1,
                    circular.cycle.join(" -> "),
                    circular.severity
                ));
            }
            output.push_str("\n");
        }
        
        // 最もインポートされているファイル
        if !analysis.most_imported_files.is_empty() {
            output.push_str("## 最もインポートされているファイル\n");
            for (file_path, count) in &analysis.most_imported_files {
                output.push_str(&format!("- {} ({}回)\n", file_path, count));
            }
            output.push_str("\n");
        }
        
        // 最も依存関係が多いファイル
        if !analysis.most_dependent_files.is_empty() {
            output.push_str("## 最も依存関係が多いファイル\n");
            for (file_path, count) in &analysis.most_dependent_files {
                output.push_str(&format!("- {} ({}個の依存関係)\n", file_path, count));
            }
            output.push_str("\n");
        }
        
        // 孤立ファイル
        if !analysis.orphaned_files.is_empty() {
            output.push_str("## 孤立ファイル\n");
            for file_path in &analysis.orphaned_files {
                output.push_str(&format!("- {}\n", file_path));
            }
            output.push_str("\n");
        }
        
        // 依存関係の深さ
        output.push_str("## 依存関係の深さ\n");
        let mut depth_entries: Vec<_> = analysis.dependency_depth.iter().collect();
        depth_entries.sort_by(|a, b| b.1.cmp(a.1));
        
        for (file_path, depth) in depth_entries.iter().take(10) {
            output.push_str(&format!("- {} (深さ: {})\n", file_path, depth));
        }
        
        Ok(output)
    }

    fn sanitize_node_id(&self, id: &str) -> String {
        id.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
}

impl Default for GraphFormatter {
    fn default() -> Self {
        Self::new()
    }
} 