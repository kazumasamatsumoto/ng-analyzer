use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_ast::*;
use swc_common::{SourceMap, BytePos};
use std::sync::Arc;
use anyhow::Result;
use crate::ast::{NgComponent, NgService, ChangeDetectionStrategy, NgInput, NgOutput, NgMethod, Parameter};
use crate::ast::{Import, Export, ImportType, ExportType, FileType};
use std::path::PathBuf;

pub struct TypeScriptParser {
    #[allow(dead_code)]
    source_map: Arc<SourceMap>,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self {
            source_map: Arc::new(SourceMap::default()),
        }
    }
    
    fn normalize_path(path: &PathBuf) -> String {
        path.display().to_string().replace('\\', "/")
    }

    pub fn parse_file(&self, content: &str) -> Result<Module> {
        let input = StringInput::new(content, BytePos(0), BytePos(content.len() as u32));
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
            }),
            EsVersion::Es2020,
            input,
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module()
            .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;
        
        Ok(module)
    }

    pub fn extract_component(&self, module: &Module, file_path: &PathBuf) -> Result<Option<NgComponent>> {
        let mut component = None;
        
        for item in &module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) = item {
                if let Decl::Class(class_decl) = &export_decl.decl {
                    if let Some(comp) = self.analyze_class_for_component(class_decl, file_path)? {
                        component = Some(comp);
                        break;
                    }
                }
            }
        }

        Ok(component)
    }

    pub fn extract_service(&self, module: &Module, file_path: &PathBuf) -> Result<Option<NgService>> {
        for item in &module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) = item {
                if let Decl::Class(class_decl) = &export_decl.decl {
                    if let Some(service) = self.analyze_class_for_service(class_decl, file_path)? {
                        return Ok(Some(service));
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn extract_imports_exports(&self, module: &Module, file_path: &PathBuf) -> Result<(Vec<Import>, Vec<Export>)> {
        let mut imports = Vec::new();
        let mut exports = Vec::new();

        for item in &module.body {
            match item {
                ModuleItem::ModuleDecl(module_decl) => {
                    match module_decl {
                        ModuleDecl::Import(import_decl) => {
                            let source_module = import_decl.src.value.to_string();
                            
                            for specifier in &import_decl.specifiers {
                                match specifier {
                                    ImportSpecifier::Named(named) => {
                                        let symbol_name = match &named.imported {
                                            Some(ModuleExportName::Ident(ident)) => ident.sym.to_string(),
                                            _ => named.local.sym.to_string(),
                                        };
                                        imports.push(Import {
                                            file_path: Self::normalize_path(file_path),
                                            symbol_name,
                                            source_module: source_module.clone(),
                                            import_type: ImportType::Named,
                                            line_number: None,
                                        });
                                    }
                                    ImportSpecifier::Default(default) => {
                                        imports.push(Import {
                                            file_path: Self::normalize_path(file_path),
                                            symbol_name: default.local.sym.to_string(),
                                            source_module: source_module.clone(),
                                            import_type: ImportType::Default,
                                            line_number: None,
                                        });
                                    }
                                    ImportSpecifier::Namespace(namespace) => {
                                        imports.push(Import {
                                            file_path: Self::normalize_path(file_path),
                                            symbol_name: namespace.local.sym.to_string(),
                                            source_module: source_module.clone(),
                                            import_type: ImportType::Namespace,
                                            line_number: None,
                                        });
                                    }
                                }
                            }
                        }
                        ModuleDecl::ExportDecl(export_decl) => {
                            match &export_decl.decl {
                                Decl::Class(class_decl) => {
                                    exports.push(Export {
                                        file_path: Self::normalize_path(file_path),
                                        symbol_name: class_decl.ident.sym.to_string(),
                                        export_type: ExportType::Named,
                                        line_number: None,
                                    });
                                }
                                Decl::Fn(fn_decl) => {
                                    exports.push(Export {
                                        file_path: Self::normalize_path(file_path),
                                        symbol_name: fn_decl.ident.sym.to_string(),
                                        export_type: ExportType::Named,
                                        line_number: None,
                                    });
                                }
                                Decl::Var(var_decl) => {
                                    for decl in &var_decl.decls {
                                        if let Pat::Ident(ident) = &decl.name {
                                            exports.push(Export {
                                                file_path: Self::normalize_path(file_path),
                                                symbol_name: ident.id.sym.to_string(),
                                                export_type: ExportType::Named,
                                                line_number: None,
                                            });
                                        }
                                    }
                                }
                                Decl::TsInterface(interface_decl) => {
                                    exports.push(Export {
                                        file_path: Self::normalize_path(file_path),
                                        symbol_name: interface_decl.id.sym.to_string(),
                                        export_type: ExportType::Named,
                                        line_number: None,
                                    });
                                }
                                Decl::TsTypeAlias(type_alias) => {
                                    exports.push(Export {
                                        file_path: Self::normalize_path(file_path),
                                        symbol_name: type_alias.id.sym.to_string(),
                                        export_type: ExportType::Named,
                                        line_number: None,
                                    });
                                }
                                Decl::TsEnum(enum_decl) => {
                                    exports.push(Export {
                                        file_path: Self::normalize_path(file_path),
                                        symbol_name: enum_decl.id.sym.to_string(),
                                        export_type: ExportType::Named,
                                        line_number: None,
                                    });
                                }
                                _ => {}
                            }
                        }
                        ModuleDecl::ExportNamed(export_named) => {
                            for specifier in &export_named.specifiers {
                                match specifier {
                                    ExportSpecifier::Named(named) => {
                                                                let symbol_name = match &named.exported {
                            Some(ModuleExportName::Ident(ident)) => ident.sym.to_string(),
                            _ => match &named.orig {
                                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                ModuleExportName::Str(s) => s.value.to_string(),
                            },
                        };
                                        exports.push(Export {
                                            file_path: Self::normalize_path(file_path),
                                            symbol_name,
                                            export_type: if export_named.src.is_some() {
                                                ExportType::ReExport
                                            } else {
                                                ExportType::Named
                                            },
                                            line_number: None,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                        ModuleDecl::ExportDefaultDecl(export_default) => {
                            let symbol_name = match &export_default.decl {
                                DefaultDecl::Class(class_expr) => {
                                    if let Some(ident) = &class_expr.ident {
                                        ident.sym.to_string()
                                    } else {
                                        "default".to_string()
                                    }
                                }
                                DefaultDecl::Fn(fn_expr) => {
                                    if let Some(ident) = &fn_expr.ident {
                                        ident.sym.to_string()
                                    } else {
                                        "default".to_string()
                                    }
                                }
                                DefaultDecl::TsInterfaceDecl(interface) => {
                                    interface.id.sym.to_string()
                                }
                            };
                            exports.push(Export {
                                file_path: Self::normalize_path(file_path),
                                symbol_name,
                                export_type: ExportType::Default,
                                line_number: None,
                            });
                        }
                        ModuleDecl::ExportDefaultExpr(_) => {
                            exports.push(Export {
                                file_path: Self::normalize_path(file_path),
                                symbol_name: "default".to_string(),
                                export_type: ExportType::Default,
                                line_number: None,
                            });
                        }
                        ModuleDecl::ExportAll(_) => {
                            exports.push(Export {
                                file_path: Self::normalize_path(file_path),
                                symbol_name: "*".to_string(),
                                export_type: ExportType::Namespace,
                                line_number: None,
                            });
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok((imports, exports))
    }

    pub fn get_file_type(&self, file_path: &PathBuf) -> FileType {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "ts" => FileType::TypeScript,
            "js" => FileType::JavaScript,
            "d.ts" => FileType::Declaration,
            _ => FileType::Module,
        }
    }

    fn analyze_class_for_component(&self, class_decl: &ClassDecl, file_path: &PathBuf) -> Result<Option<NgComponent>> {
        let mut selector = None;
        let mut template_url = None;
        let mut template = None;
        let mut style_urls = Vec::new();
        let mut change_detection = ChangeDetectionStrategy::Default;

        if !class_decl.class.decorators.is_empty() {
            for decorator in &class_decl.class.decorators {
                if let Expr::Call(call_expr) = &*decorator.expr {
                    if let Callee::Expr(expr) = &call_expr.callee {
                        if let Expr::Ident(ident) = &**expr {
                            if ident.sym.as_ref() == "Component" {
                                if let Some(args) = call_expr.args.first() {
                                    if let Expr::Object(obj_lit) = &*args.expr {
                                        for prop in &obj_lit.props {
                                            if let PropOrSpread::Prop(prop) = prop {
                                                self.extract_component_metadata(&**prop, &mut selector, &mut template_url, &mut template, &mut style_urls, &mut change_detection);
                                            }
                                        }
                                    }
                                }

                                let inputs = self.extract_inputs(&class_decl.class)?;
                                let outputs = self.extract_outputs(&class_decl.class)?;
                                let lifecycle_hooks = self.extract_lifecycle_hooks(&class_decl.class)?;
                                let dependencies = self.extract_dependencies(&class_decl.class)?;
                                let complexity_score = self.calculate_complexity(&class_decl.class)?;

                                return Ok(Some(NgComponent {
                                    name: class_decl.ident.sym.to_string(),
                                    file_path: Self::normalize_path(file_path),
                                    selector,
                                    template_url,
                                    template,
                                    style_urls,
                                    inputs,
                                    outputs,
                                    lifecycle_hooks,
                                    dependencies,
                                    change_detection,
                                    complexity_score,
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn analyze_class_for_service(&self, class_decl: &ClassDecl, file_path: &PathBuf) -> Result<Option<NgService>> {
        let mut provided_in = None;
        let mut injectable = false;

        if !class_decl.class.decorators.is_empty() {
            for decorator in &class_decl.class.decorators {
                if let Expr::Call(call_expr) = &*decorator.expr {
                    if let Callee::Expr(expr) = &call_expr.callee {
                        if let Expr::Ident(ident) = &**expr {
                            if ident.sym.as_ref() == "Injectable" {
                                injectable = true;
                                if let Some(args) = call_expr.args.first() {
                                    if let Expr::Object(obj_lit) = &*args.expr {
                                        for prop in &obj_lit.props {
                                            if let PropOrSpread::Prop(prop) = prop {
                                                if let Prop::KeyValue(kv) = &**prop {
                                                    if let PropName::Ident(key) = &kv.key {
                                                        if key.sym.as_ref() == "providedIn" {
                                                            if let Expr::Lit(Lit::Str(str_lit)) = &*kv.value {
                                                                provided_in = Some(str_lit.value.to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if injectable {
            let dependencies = self.extract_dependencies(&class_decl.class)?;
            let methods = self.extract_methods(&class_decl.class)?;

            return Ok(Some(NgService {
                name: class_decl.ident.sym.to_string(),
                file_path: Self::normalize_path(file_path),
                provided_in,
                injectable,
                dependencies,
                methods,
            }));
        }

        Ok(None)
    }

    fn extract_component_metadata(
        &self,
        prop: &Prop,
        selector: &mut Option<String>,
        template_url: &mut Option<String>,
        template: &mut Option<String>,
        style_urls: &mut Vec<String>,
        change_detection: &mut ChangeDetectionStrategy,
    ) {
        if let Prop::KeyValue(kv) = prop {
            if let PropName::Ident(key) = &kv.key {
                match key.sym.as_ref() {
                    "selector" => {
                        if let Expr::Lit(Lit::Str(str_lit)) = &*kv.value {
                            *selector = Some(str_lit.value.to_string());
                        }
                    }
                    "templateUrl" => {
                        if let Expr::Lit(Lit::Str(str_lit)) = &*kv.value {
                            *template_url = Some(str_lit.value.to_string());
                        }
                    }
                    "template" => {
                        if let Expr::Lit(Lit::Str(str_lit)) = &*kv.value {
                            *template = Some(str_lit.value.to_string());
                        }
                    }
                    "styleUrls" => {
                        if let Expr::Array(arr_lit) = &*kv.value {
                            for elem in &arr_lit.elems {
                                if let Some(ExprOrSpread { expr, .. }) = elem {
                                    if let Expr::Lit(Lit::Str(str_lit)) = &**expr {
                                        style_urls.push(str_lit.value.to_string());
                                    }
                                }
                            }
                        }
                    }
                    "changeDetection" => {
                        if let Expr::Member(member_expr) = &*kv.value {
                            if let MemberProp::Ident(ident) = &member_expr.prop {
                                if ident.sym.as_ref() == "OnPush" {
                                    *change_detection = ChangeDetectionStrategy::OnPush;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn extract_inputs(&self, class: &Class) -> Result<Vec<NgInput>> {
        let mut inputs = Vec::new();
        
        for member in &class.body {
            if let ClassMember::ClassProp(prop) = member {
                for decorator in &prop.decorators {
                    if let Expr::Call(call_expr) = &*decorator.expr {
                        if let Callee::Expr(expr) = &call_expr.callee {
                            if let Expr::Ident(ident) = &**expr {
                                if ident.sym.as_ref() == "Input" {
                                    if let PropName::Ident(ident) = &prop.key {
                                        inputs.push(NgInput {
                                            name: ident.sym.to_string(),
                                            alias: None,
                                            input_type: "any".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(inputs)
    }

    fn extract_outputs(&self, class: &Class) -> Result<Vec<NgOutput>> {
        let mut outputs = Vec::new();
        
        for member in &class.body {
            if let ClassMember::ClassProp(prop) = member {
                for decorator in &prop.decorators {
                    if let Expr::Call(call_expr) = &*decorator.expr {
                        if let Callee::Expr(expr) = &call_expr.callee {
                            if let Expr::Ident(ident) = &**expr {
                                if ident.sym.as_ref() == "Output" {
                                    if let PropName::Ident(ident) = &prop.key {
                                        outputs.push(NgOutput {
                                            name: ident.sym.to_string(),
                                            alias: None,
                                            output_type: "EventEmitter<any>".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(outputs)
    }

    fn extract_lifecycle_hooks(&self, class: &Class) -> Result<Vec<String>> {
        let mut hooks = Vec::new();
        let lifecycle_methods = vec![
            "ngOnInit", "ngOnDestroy", "ngOnChanges", "ngAfterViewInit",
            "ngAfterViewChecked", "ngAfterContentInit", "ngAfterContentChecked",
            "ngDoCheck"
        ];

        for member in &class.body {
            if let ClassMember::Method(method) = member {
                if let PropName::Ident(ident) = &method.key {
                    let method_name = ident.sym.to_string();
                    if lifecycle_methods.contains(&method_name.as_str()) {
                        hooks.push(method_name);
                    }
                }
            }
        }

        Ok(hooks)
    }

    fn extract_dependencies(&self, class: &Class) -> Result<Vec<String>> {
        let mut dependencies = Vec::new();

        for member in &class.body {
            if let ClassMember::Constructor(constructor) = member {
                for param in &constructor.params {
                    if let ParamOrTsParamProp::TsParamProp(ts_param) = param {
                        if let TsParamPropParam::Ident(ident) = &ts_param.param {
                            if let Some(type_ann) = &ident.type_ann {
                                dependencies.push(self.extract_type_from_annotation(&type_ann.type_ann));
                            }
                        }
                    }
                }
            }
        }

        Ok(dependencies)
    }

    fn extract_methods(&self, class: &Class) -> Result<Vec<NgMethod>> {
        let mut methods = Vec::new();

        for member in &class.body {
            if let ClassMember::Method(method) = member {
                if let PropName::Ident(ident) = &method.key {
                    let method_name = ident.sym.to_string();
                    if !method_name.starts_with("ng") {
                        let parameters = method.function.params.iter()
                            .map(|_param| Parameter {
                                name: "param".to_string(),
                                param_type: "any".to_string(),
                                optional: false,
                            })
                            .collect();

                        methods.push(NgMethod {
                            name: method_name,
                            parameters,
                            return_type: None,
                            complexity_score: 1,
                        });
                    }
                }
            }
        }

        Ok(methods)
    }

    fn calculate_complexity(&self, class: &Class) -> Result<u32> {
        let mut complexity = 1;

        for member in &class.body {
            if let ClassMember::Method(method) = member {
                complexity += self.calculate_method_complexity(&method.function);
            }
        }

        Ok(complexity)
    }

    fn calculate_method_complexity(&self, _function: &Function) -> u32 {
        1
    }

    fn extract_type_from_annotation(&self, ts_type: &TsType) -> String {
        match ts_type {
            TsType::TsTypeRef(type_ref) => {
                if let TsEntityName::Ident(ident) = &type_ref.type_name {
                    ident.sym.to_string()
                } else {
                    "unknown".to_string()
                }
            }
            _ => "unknown".to_string(),
        }
    }
}