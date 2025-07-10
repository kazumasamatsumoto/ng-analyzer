use crate::ast::NgProject;
use crate::parsers::typescript::TypeScriptParser;
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use ignore::WalkBuilder;

pub struct ProjectParser {
    typescript_parser: TypeScriptParser,
}

impl ProjectParser {
    pub fn new() -> Self {
        Self {
            typescript_parser: TypeScriptParser::new(),
        }
    }

    pub async fn parse_project(&self, root_path: &PathBuf) -> Result<NgProject> {
        let mut project = NgProject {
            root_path: root_path.clone(),
            ..Default::default()
        };

        let walker = WalkBuilder::new(root_path)
            .add_custom_ignore_filename(".ngignore")
            .hidden(false)
            .git_ignore(true)
            .build();

        for entry in walker {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("ts") => {
                            if path.to_string_lossy().contains(".component.") {
                                if let Some(component) = self.parse_component_file(path).await? {
                                    project.components.push(component);
                                }
                            } else if path.to_string_lossy().contains(".service.") {
                                if let Some(service) = self.parse_service_file(path).await? {
                                    project.services.push(service);
                                }
                            } else if path.to_string_lossy().contains(".module.") {
                                if let Some(module) = self.parse_module_file(path).await? {
                                    project.modules.push(module);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(project)
    }

    async fn parse_component_file(&self, file_path: &std::path::Path) -> Result<Option<crate::ast::NgComponent>> {
        let content = fs::read_to_string(file_path)?;
        let _module = self.typescript_parser.parse_file(&content)?;
        
        self.typescript_parser.extract_component(&_module, &file_path.to_path_buf())
    }

    async fn parse_service_file(&self, file_path: &std::path::Path) -> Result<Option<crate::ast::NgService>> {
        let content = fs::read_to_string(file_path)?;
        let _module = self.typescript_parser.parse_file(&content)?;
        
        self.typescript_parser.extract_service(&_module, &file_path.to_path_buf())
    }

    async fn parse_module_file(&self, file_path: &std::path::Path) -> Result<Option<crate::ast::NgModule>> {
        let content = fs::read_to_string(file_path)?;
        let _module = self.typescript_parser.parse_file(&content)?;
        
        Ok(Some(crate::ast::NgModule {
            name: file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            file_path: file_path.to_path_buf(),
            imports: Vec::new(),
            exports: Vec::new(),
            declarations: Vec::new(),
            providers: Vec::new(),
            bootstrap: Vec::new(),
        }))
    }
}