use crate::ast::{ImportExportGraph, DependencyAnalysis, FileInfo, Dependency, CircularDependency, CycleSeverity};
use crate::parsers::typescript::TypeScriptParser;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use ignore::WalkBuilder;

pub struct DependencyGraphAnalyzer {
    typescript_parser: TypeScriptParser,
}

impl DependencyGraphAnalyzer {
    pub fn new() -> Self {
        Self {
            typescript_parser: TypeScriptParser::new(),
        }
    }

    pub async fn analyze_project(&self, root_path: &PathBuf) -> Result<ImportExportGraph> {
        let mut graph = ImportExportGraph::default();
        let mut file_id_counter = 0;

        // プロジェクト内のすべてのTypeScriptファイルを走査
        let walker = WalkBuilder::new(root_path)
            .hidden(false)
            .git_ignore(true)
            .add_custom_ignore_filename(".gitignore")
            .build();

        for entry in walker {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if matches!(extension.to_str(), Some("ts") | Some("js") | Some("tsx") | Some("jsx")) {
                        if let Ok(content) = fs::read_to_string(path) {
                            if let Ok(module) = self.typescript_parser.parse_file(&content) {
                                let file_path = path.to_path_buf();
                                let relative_path = path.strip_prefix(root_path)
                                    .unwrap_or(path)
                                    .to_string_lossy()
                                    .to_string();
                                
                                let (imports, exports) = self.typescript_parser.extract_imports_exports(&module, &file_path)?;
                                
                                // FileInfo を追加
                                let file_id = format!("file_{}", file_id_counter);
                                file_id_counter += 1;
                                
                                graph.files.push(FileInfo {
                                    id: file_id.clone(),
                                    file_path: file_path.display().to_string(),
                                    relative_path,
                                    file_type: self.typescript_parser.get_file_type(&file_path),
                                    exports: exports.iter().map(|e| e.symbol_name.clone()).collect(),
                                    imports: imports.iter().map(|i| i.symbol_name.clone()).collect(),
                                });

                                // Imports と Exports を追加
                                graph.imports.extend(imports);
                                graph.exports.extend(exports);
                            }
                        }
                    }
                }
            }
        }

        // 依存関係を構築
        self.build_dependencies(&mut graph, root_path)?;

        Ok(graph)
    }

    fn build_dependencies(&self, graph: &mut ImportExportGraph, _root_path: &PathBuf) -> Result<()> {
        let mut path_to_file_id: HashMap<String, String> = HashMap::new();
        
        // ファイルパスとIDのマッピングを作成（パスを正規化）
        for file_info in &graph.files {
            let normalized_path = file_info.file_path.replace('\\', "/");
            path_to_file_id.insert(normalized_path, file_info.id.clone());
        }

        // 各importに対して依存関係を作成（相対パスのimportのみ処理）
        for import in &graph.imports {
            // 外部ライブラリのimportは無視
            if !import.source_module.starts_with('.') {
                continue;
            }
            
            // ファイル名ベースでマッチングする
            let mut target_file_id: Option<String> = None;
            
            // インポートパスから期待されるファイル名を抽出
            let import_target = import.source_module.trim_start_matches("./");
            let expected_filename = format!("{}.ts", import_target);
            
            // すべてのファイルから一致するものを探す
            for file_info in &graph.files {
                if let Some(filename) = Path::new(&file_info.file_path).file_name() {
                    if filename.to_string_lossy() == expected_filename {
                        target_file_id = Some(file_info.id.clone());
                        break;
                    }
                }
            }
            
            if let Some(target_file_id) = target_file_id {
                let normalized_import_path = import.file_path.replace('\\', "/");
                if let Some(source_file_id) = path_to_file_id.get(&normalized_import_path) {
                    // 重複チェック
                    let dependency_exists = graph.dependencies.iter().any(|dep| 
                        dep.from_file == *source_file_id && dep.to_file == target_file_id
                    );
                    
                    if !dependency_exists {
                        graph.dependencies.push(Dependency {
                            from_file: source_file_id.clone(),
                            to_file: target_file_id.clone(),
                            import_type: import.import_type.clone(),
                            imported_symbols: vec![import.symbol_name.clone()],
                            line_number: import.line_number,
                        });
                    } else {
                        // 既存の依存関係にシンボルを追加
                        if let Some(existing_dep) = graph.dependencies.iter_mut().find(|dep| 
                            dep.from_file == *source_file_id && dep.to_file == target_file_id
                        ) {
                            if !existing_dep.imported_symbols.contains(&import.symbol_name) {
                                existing_dep.imported_symbols.push(import.symbol_name.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_import_path(&self, import_path: &str, current_file: &str, _root_path: &PathBuf) -> Option<String> {
        let current_dir = Path::new(current_file).parent().unwrap_or(Path::new(""));
        
        if import_path.starts_with('.') {
            // 相対パス
            let mut resolved = current_dir.join(import_path);
            
            // 拡張子を追加
            if resolved.extension().is_none() {
                resolved = resolved.with_extension("ts");
            }
            
            // パスを正規化
            if let Ok(canonical) = resolved.canonicalize() {
                return Some(canonical.display().to_string().replace('\\', "/"));
            }
            
            // ファイルが存在しない場合は、拡張子なしで試す
            resolved = current_dir.join(import_path);
            if let Ok(canonical) = resolved.canonicalize() {
                return Some(canonical.display().to_string().replace('\\', "/"));
            }
            
            // それでも見つからない場合は、手動でパスを構築
            let normalized = current_dir.join(import_path).with_extension("ts");
            return Some(normalized.display().to_string().replace('\\', "/"));
        }
        
        // 外部ライブラリの場合は無視
        None
    }

    pub fn analyze_dependencies(&self, graph: &ImportExportGraph) -> Result<DependencyAnalysis> {
        let mut analysis = DependencyAnalysis::default();
        
        // 循環依存の検出
        analysis.circular_dependencies = self.find_circular_dependencies(graph)?;
        
        // 孤立したファイルの検出
        analysis.orphaned_files = self.find_orphaned_files(graph)?;
        
        // 依存関係の深さを計算
        analysis.dependency_depth = self.calculate_dependency_depth(graph)?;
        
        // 最も多く利用されているファイルを計算
        analysis.most_imported_files = self.find_most_imported_files(graph)?;
        
        // 最も多くの依存関係を持つファイルを計算
        analysis.most_dependent_files = self.find_most_dependent_files(graph)?;
        
        Ok(analysis)
    }

    fn find_circular_dependencies(&self, graph: &ImportExportGraph) -> Result<Vec<CircularDependency>> {
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        // 各ファイルからDFSを開始
        for file in &graph.files {
            if !visited.contains(&file.id) {
                let mut path = Vec::new();
                if let Some(cycle) = self.dfs_find_cycle(&file.id, graph, &mut visited, &mut rec_stack, &mut path) {
                    let severity = if cycle.len() <= 2 {
                        CycleSeverity::Critical
                    } else if cycle.len() <= 4 {
                        CycleSeverity::Warning
                    } else {
                        CycleSeverity::Info
                    };
                    
                    circular_deps.push(CircularDependency {
                        cycle,
                        severity,
                    });
                }
            }
        }
        
        Ok(circular_deps)
    }

    fn dfs_find_cycle(
        &self,
        node: &str,
        graph: &ImportExportGraph,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        // 隣接ノードを探索
        for dependency in &graph.dependencies {
            if dependency.from_file == node {
                let next_node = &dependency.to_file;
                
                if !visited.contains(next_node) {
                    if let Some(cycle) = self.dfs_find_cycle(next_node, graph, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(next_node) {
                    // 循環を発見
                    let cycle_start = path.iter().position(|x| x == next_node).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(next_node.to_string());
                    return Some(cycle);
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }

    fn find_orphaned_files(&self, graph: &ImportExportGraph) -> Result<Vec<String>> {
        let mut imported_files = HashSet::new();
        let mut exporting_files = HashSet::new();
        
        // importされているファイルを収集
        for dependency in &graph.dependencies {
            imported_files.insert(dependency.to_file.clone());
        }
        
        // exportしているファイルを収集
        for export in &graph.exports {
            exporting_files.insert(export.file_path.clone());
        }
        
        // どこからもimportされておらず、exportもしていないファイルを探す
        let orphaned: Vec<String> = graph.files.iter()
            .filter(|file| !imported_files.contains(&file.id) && !exporting_files.contains(&file.file_path))
            .map(|file| file.file_path.clone())
            .collect();
        
        Ok(orphaned)
    }

    fn calculate_dependency_depth(&self, graph: &ImportExportGraph) -> Result<HashMap<String, u32>> {
        let mut depth_map = HashMap::new();
        
        for file in &graph.files {
            let depth = self.calculate_file_depth(&file.id, graph, &mut HashSet::new())?;
            depth_map.insert(file.file_path.clone(), depth);
        }
        
        Ok(depth_map)
    }

    fn calculate_file_depth(&self, file_id: &str, graph: &ImportExportGraph, visited: &mut HashSet<String>) -> Result<u32> {
        if visited.contains(file_id) {
            return Ok(0); // 循環を避ける
        }
        
        visited.insert(file_id.to_string());
        
        let mut max_depth = 0;
        
        for dependency in &graph.dependencies {
            if dependency.from_file == file_id {
                let child_depth = self.calculate_file_depth(&dependency.to_file, graph, visited)?;
                max_depth = max_depth.max(child_depth + 1);
            }
        }
        
        visited.remove(file_id);
        Ok(max_depth)
    }

    fn find_most_imported_files(&self, graph: &ImportExportGraph) -> Result<Vec<(String, u32)>> {
        let mut import_count = HashMap::new();
        
        for dependency in &graph.dependencies {
            let count = import_count.entry(dependency.to_file.clone()).or_insert(0);
            *count += 1;
        }
        
        let mut sorted_files: Vec<(String, u32)> = import_count.into_iter()
            .map(|(file_id, count)| {
                let file_path = graph.files.iter()
                    .find(|f| f.id == file_id)
                    .map(|f| f.file_path.clone())
                    .unwrap_or(file_id);
                (file_path, count)
            })
            .collect();
        
        sorted_files.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_files.truncate(10);
        
        Ok(sorted_files)
    }

    fn find_most_dependent_files(&self, graph: &ImportExportGraph) -> Result<Vec<(String, u32)>> {
        let mut dependency_count = HashMap::new();
        
        for dependency in &graph.dependencies {
            let count = dependency_count.entry(dependency.from_file.clone()).or_insert(0);
            *count += 1;
        }
        
        let mut sorted_files: Vec<(String, u32)> = dependency_count.into_iter()
            .map(|(file_id, count)| {
                let file_path = graph.files.iter()
                    .find(|f| f.id == file_id)
                    .map(|f| f.file_path.clone())
                    .unwrap_or(file_id);
                (file_path, count)
            })
            .collect();
        
        sorted_files.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_files.truncate(10);
        
        Ok(sorted_files)
    }
}

impl Default for DependencyGraphAnalyzer {
    fn default() -> Self {
        Self::new()
    }
} 