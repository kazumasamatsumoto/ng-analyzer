use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::driver::ParseOpts;
use markup5ever_rcdom::{RcDom, NodeData};
use anyhow::Result;
use std::io::Cursor;

pub struct HtmlParser;

impl HtmlParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_template(&self, template: &str) -> Result<TemplateAnalysis> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut Cursor::new(template.as_bytes()))?;

        let mut analysis = TemplateAnalysis::default();
        self.analyze_node(&dom.document, &mut analysis)?;

        Ok(analysis)
    }

    fn analyze_node(&self, node: &markup5ever_rcdom::Handle, analysis: &mut TemplateAnalysis) -> Result<()> {
        match &node.data {
            NodeData::Element { name, attrs, .. } => {
                let tag_name = name.local.to_string();
                analysis.elements.push(tag_name.clone());

                let attrs = attrs.borrow();
                for attr in attrs.iter() {
                    let attr_name = attr.name.local.to_string();
                    if attr_name.starts_with('(') && attr_name.ends_with(')') {
                        analysis.event_bindings.push(attr_name);
                    } else if attr_name.starts_with('[') && attr_name.ends_with(']') {
                        analysis.property_bindings.push(attr_name);
                    } else if attr_name.starts_with("*ng") {
                        analysis.structural_directives.push(attr_name);
                    }
                }
            }
            _ => {}
        }

        for child in node.children.borrow().iter() {
            self.analyze_node(child, analysis)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct TemplateAnalysis {
    pub elements: Vec<String>,
    pub event_bindings: Vec<String>,
    pub property_bindings: Vec<String>,
    pub structural_directives: Vec<String>,
    pub interpolations: Vec<String>,
}