use anyhow::Result;
use html5ever::driver::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

#[allow(dead_code)]
pub struct HtmlParser;

impl HtmlParser {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn parse_template(&self, template: &str) -> Result<TemplateAnalysis> {
        let mut analysis = TemplateAnalysis {
            elements: Vec::new(),
            event_bindings: Vec::new(),
            property_bindings: Vec::new(),
            structural_directives: Vec::new(),
            interpolations: Vec::new(),
        };

        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut template.as_bytes())?;

        self.analyze_node(&dom.document, &mut analysis)?;

        Ok(analysis)
    }

    #[allow(dead_code)]
    fn analyze_node(&self, node: &markup5ever_rcdom::Handle, analysis: &mut TemplateAnalysis) -> Result<()> {
        match &node.data {
            markup5ever_rcdom::NodeData::Element { name, attrs, .. } => {
                let element_name = name.local.to_string();
                analysis.elements.push(element_name);
                
                for attr in attrs.borrow().iter() {
                    let attr_name = attr.name.local.to_string();
                    let attr_value = attr.value.to_string();
                    
                    if attr_name.starts_with("(") && attr_name.ends_with(")") {
                        analysis.event_bindings.push(format!("{}={}", attr_name, attr_value));
                    } else if attr_name.starts_with("[") && attr_name.ends_with("]") {
                        analysis.property_bindings.push(format!("{}={}", attr_name, attr_value));
                    } else if attr_name.starts_with("*") {
                        analysis.structural_directives.push(format!("{}={}", attr_name, attr_value));
                    }
                }
            }
            markup5ever_rcdom::NodeData::Text { contents } => {
                let text = contents.borrow().to_string();
                if text.contains("{{") && text.contains("}}") {
                    analysis.interpolations.push(text);
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

#[derive(Debug)]
pub struct TemplateAnalysis {
    #[allow(dead_code)]
    pub elements: Vec<String>,
    #[allow(dead_code)]
    pub event_bindings: Vec<String>,
    #[allow(dead_code)]
    pub property_bindings: Vec<String>,
    #[allow(dead_code)]
    pub structural_directives: Vec<String>,
    #[allow(dead_code)]
    pub interpolations: Vec<String>,
}