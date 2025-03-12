#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]

/*!
# hlight

A library for output syntax highlighting.
*/
pub mod output;
mod resource;
pub mod syntax;
pub mod theme;

pub use crate::{output::Highlighter, resource::HighlightResource};
