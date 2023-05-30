use getset::{Getters, MutGetters};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
pub use syntect::{dumps, highlighting::ThemeSet};
use syntect::{highlighting::Theme, parsing::SyntaxSet};

use crate::theme::theme_monokai;

#[derive(Getters, MutGetters, Debug, Clone)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLightRes<'name> {
    name: Cow<'name, str>,
    theme: OnceCell<Theme>,
    theme_set: &'name ThemeSet,
    syntax_set: &'static SyntaxSet,
    background: bool,
}

impl<'name> HighLightRes<'name> {
    /// Create a new instance of HighLightRes
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hlight::HighLightRes;
    /// use hlight::theme::load_theme_set;
    /// use std::borrow::Cow;
    ///
    /// const THEMES: &[u8] = include_bytes!(concat!(
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "/assets/theme-syntax-set/theme-set.packdump"
    /// ));
    ///
    /// let set = load_theme_set(Some(THEMES));
    /// let res = HighLightRes::new(Cow::from("ayu-dark"), &set);
    /// ```
    pub fn new(name: Cow<'name, str>, theme_set: &'name ThemeSet) -> Self {
        Self {
            name,
            theme_set,
            syntax_set: Self::static_syntax_set(),
            ..Default::default()
        }
    }

    /// Enable or disable background
    ///
    /// # Example
    ///
    /// ```
    /// use hlight::HighLightRes;
    ///
    /// let res = HighLightRes::default().with_background(false);
    /// assert!(!res.get_background())
    /// ```
    pub fn with_background(self, switch: bool) -> Self {
        Self {
            background: switch,
            ..self
        }
    }
}

impl<'name> Default for HighLightRes<'name> {
    fn default() -> Self {
        Self {
            name: theme_monokai(),
            theme: OnceCell::new(),
            syntax_set: Self::static_syntax_set(),
            theme_set: Self::static_theme_set(),
            background: true,
        }
    }
}
