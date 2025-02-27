use std::cell::OnceCell;

use getset::{Getters, WithSetters};
use syntect::{
  highlighting::{Theme, ThemeSet},
  parsing::SyntaxSet,
};

use crate::theme::{self, names::CmString};

/// HighLight Resource
///
/// ## Create new instance
///
/// ```
/// use hlight::HighLightRes;
/// use hlight::theme::names::monokai;
///
/// let res = HighLightRes::default();
/// assert_eq!(res.get_name(), monokai());
/// ```
///
/// ## Enable or disable background
///
/// ```
/// use hlight::HighLightRes;
///
/// let res = HighLightRes::default().with_background(false);
/// assert!(!res.get_background())
/// ```
#[derive(Getters, WithSetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct HighLightRes<'theme> {
  name: CmString,
  #[getset(get = "pub(crate)")]
  theme: OnceCell<Theme>,
  theme_set: &'theme ThemeSet,
  syntax_set: &'static SyntaxSet,
  background: bool,
}

impl<'a> HighLightRes<'a> {
  /// Creates a new instance of HighLightRes
  ///
  /// ### Example
  ///
  /// ```no_run
  /// use hlight::HighLightRes;
  /// use hlight::theme::load_theme_set;
  /// use std::borrow::Cow;
  ///
  /// const THEMES: &[u8] = include_bytes!(concat!(
  ///     env!("CARGO_MANIFEST_DIR"),
  ///     "/assets/set/theme.packdump"
  /// ));
  ///
  /// let set = load_theme_set(Some(THEMES));
  /// let res = HighLightRes::new("ayu-dark", &set);
  /// ```
  pub fn new(name: &str, theme_set: &'a ThemeSet) -> Self {
    Self {
      name: name.into(),
      theme_set,
      syntax_set: Self::static_syntax_set(),
      ..Default::default()
    }
  }
}

impl Default for HighLightRes<'_> {
  fn default() -> Self {
    Self {
      name: theme::names::monokai(),
      theme: OnceCell::new(),
      syntax_set: Self::static_syntax_set(),
      theme_set: Self::static_theme_set(),
      background: true,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::theme::names;

  #[test]
  #[ignore]
  /// Create two different HighLightRes instances, then initialize the name
  /// field (via `.get_theme_or_init_once()`) to see if there is any impact
  /// between the two structure instances.
  ///
  /// Note: The test result is no.
  fn test_once_cell_name() {
    macro_rules! assert_theme_name {
      ($res:expr, $expected:expr) => {
        assert_eq!(
          &$res
            .get_theme_or_init_once()
            .name
            .as_deref(),
          &Some($expected)
        );
      };
    }

    let res = HighLightRes::default();
    assert_theme_name!(res, names::monokai().as_str());

    let res2 = HighLightRes::default().with_name(names::ayu_dark());
    assert_theme_name!(res2, "ayu");
  }
}
