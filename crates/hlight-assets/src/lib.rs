#![no_std]
#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
//! Provides preset syntax and theme sets for hlight.
//!
//! Note: This library requires hlight to function properly.
#[cfg(feature = "syntax-set")]
pub const SUBLIME_SYNTAXES: &[u8] = include_bytes!(concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/assets/set/syntax.packdump"
));

#[cfg(feature = "theme-set")]
pub const THEME_SET: &[u8] = include_bytes!(concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/assets/set/theme.packdump"
));
