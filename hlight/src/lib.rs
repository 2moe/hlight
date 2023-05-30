// cargo +nightly rustdoc --all-features -- --cfg __hlight_doc --document-private-items ; open $CARGO_TARGET_DIR/doc/hlight/index.html
#![cfg_attr(__hlight_doc, feature(doc_auto_cfg, doc_notable_trait))]

//! # hlight
//!
//! A library for output syntax highlighting.
//!
//! ## Quick Start
//!
//! ### add dep
//!
//! ```sh
//! cargo add hlight
//! ```
//!
//! ### print to stdout
//!
//! ```rust
//! use hlight::{get_syntax_highlight, theme::theme_ayu_dark, HighLightRes};
//!
//!
//! let s: &str = r#"
//! [main]
//! enabled = false
//! "😎" = "🍥"
//! float = nan
//! "#;
//!
//! let mut res = HighLightRes::default().with_background(false);
//! // theme_ayu_dark: Cow::from("ayu-dark")
//! *res.get_name_mut() = theme_ayu_dark();
//!
//! get_syntax_highlight("toml", s, Some(&res), None)
//!     .expect("Failed to get highlighted toml text");
//! ```
//!
//! output:
//!
//! <pre>
//!     <span style="color:#BFBAB0">[</span><span style="color:#A6E22E">main</span><span style="color:#BFBAB0">]</span>
//!     <span style="color:#59C2FF">enabled</span><span style="color:#BFBAB0"> = </span><span style="color:#F29718">false</span>
//!     <span style="color:#C2D94C">&quot;aa&quot;</span><span style="color:#BFBAB0"> = </span><span style="color:#C2D94C">&quot;bb&quot;</span>
//!     <span style="color:#59C2FF">float</span><span style="color:#BFBAB0"> = </span><span style="color:#F29718">nan</span>
//! </pre>
//!
//! <hr />
//!
//! ```no_run
//! use std::borrow::Cow;
//! *res.get_name_mut() = Cow::from("OneHalfLight");
//!
//! get_syntax_highlight("toml", s, Some(&res), None)?;
//! ```
//!
//! output:
//!
//! <pre>
//!     <span style="color:#383A42">[main]</span>
//!     <span style="color:#E45649">enabled</span><span style="color:#383A42"> = </span><span style="color:#C18401">false</span>
//!     <span style="color:#50A14F">&quot;😎&quot;</span><span style="color:#383A42"> = </span><span style="color:#50A14F">&quot;🍥&quot;</span>
//!     <span style="color:#E45649">float</span><span style="color:#383A42"> = </span><span style="color:#C18401">nan</span>
//! </pre>
//!
//! ### write to file
//!
//! ```rust
//! use std::fs::File;
//!
//! let mut file = File::create("test.txt").expect("Failed to create test.txt");
//! get_syntax_highlight("toml", s, Some(&res), Some(&mut file))
//!     .expect("Unable to write syntax-highlighted text to file.")
//! ```
mod output;
mod resource;
pub mod syntax;
pub mod theme;

pub use crate::{output::get_syntax_highlight, resource::HighLightRes};
