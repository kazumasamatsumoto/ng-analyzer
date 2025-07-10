pub mod typescript;
pub mod html;
pub mod project;

use crate::ast::NgProject;
use anyhow::Result;
use std::path::PathBuf;

pub use typescript::TypeScriptParser;
pub use html::HtmlParser;
pub use project::ProjectParser;