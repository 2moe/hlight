use std::{borrow::Cow, sync::OnceLock};

use getset::{Getters, WithSetters};
use syntect::{highlighting::Theme, parsing::SyntaxSet};

use crate::theme::{self, HlightThemeSet, names::CmString};

/// Highlight Resource
///
/// ## Create new instance
///
/// ```
/// use hlight::HighlightResource;
/// use hlight::theme::names::monokai;
///
/// let res = HighlightResource::default();
/// assert_eq!(res.get_theme_name(), monokai());
/// ```
///
/// ## Enable or disable background
///
/// ```
/// use hlight::HighlightResource;
///
/// let res = HighlightResource::default().with_background(false);
/// assert!(!res.get_background())
/// ```
#[derive(Getters, WithSetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct HighlightResource<'theme> {
  theme_name: CmString,
  #[getset(get = "pub(crate)")]
  /// - get or init: [Self::get_or_init_theme]
  theme: OnceLock<Theme>,
  theme_set: HlightThemeSet<'theme>,
  syntax_set: Cow<'theme, SyntaxSet>, //&'theme SyntaxSet,
  background: bool,
}

impl Default for HighlightResource<'_> {
  fn default() -> Self {
    Self {
      theme_name: theme::names::monokai(),
      theme: OnceLock::new(),
      syntax_set: Cow::Borrowed(Self::static_syntax_set()),
      theme_set: Self::static_theme_set().into(),
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
  /// Create two different HighlightResource instances, then initialize the name
  /// field (via `.get_or_init_theme()`) to see if there is any impact
  /// between the two structure instances.
  ///
  /// Note: The test result is no.
  fn test_once_cell_name() {
    macro_rules! assert_theme_name {
      ($res:expr, $expected:expr) => {
        assert_eq!(
          &$res
            .get_or_init_theme()
            .name
            .as_deref(),
          &Some($expected)
        );
      };
    }

    let res = HighlightResource::default();
    assert_theme_name!(res, names::monokai().as_str());

    let res2 = HighlightResource::default().with_theme_name(names::ayu_dark());
    assert_theme_name!(res2, "ayu");
  }
}
