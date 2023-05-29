use getset::{Getters, MutGetters};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
pub use syntect::{dumps, highlighting::ThemeSet};
use syntect::{highlighting::Theme, parsing::SyntaxSet};

use crate::theme::theme_monokai;

#[derive(Getters, MutGetters, Debug)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct HighLightRes<'name> {
    name: Cow<'name, str>,
    theme: OnceCell<Theme>,
    theme_set: &'name ThemeSet,
    syntax_set: &'static SyntaxSet,
    background: bool,
}

impl<'name> HighLightRes<'name> {
    pub fn new(name: Cow<'name, str>, theme_set: &'name ThemeSet) -> Self {
        Self {
            name,
            theme_set,
            syntax_set: Self::static_syntax_set(),
            // theme: OnceCell::new(),
            // background: true,
            ..Default::default()
        }
    }

    /// Enable or disable background
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
