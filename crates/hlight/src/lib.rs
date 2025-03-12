// cargo +nightly rustdoc --all-features -- --cfg __unstable_doc
// --document-private-items ; open $CARGO_TARGET_DIR/doc/hlight/index.html
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
