use std::sync::LazyLock;

pub use syntect::{
  dumps,
  highlighting::{Theme, ThemeSet},
};

use crate::resource::HighlightResource;

pub const READ_DUMP_DATA_ERR: &str = "Failed to read dump data";

/// Some theme names included in the "preset-theme-set".
///
/// You don't need to enable the "preset-theme-set" feature to use this module.
/// If you load a custom theme set, use the theme names from your theme set
/// instead of this module.
pub mod names {
  pub use compact_str::CompactString as CmString;

  /// [CompactString::const_new](CmString::const_new)
  #[inline]
  pub const fn new(s: &'static str) -> CmString {
    CmString::const_new(s)
  }

  /// "Monokai Extended"
  pub const fn monokai() -> CmString {
    new("Monokai Extended")
  }

  /// "ayu-dark"
  pub const fn ayu_dark() -> CmString {
    new("ayu-dark")
  }
}
/// Loads a set of themes.
///
/// If the parameter is None, the default theme set is used.
///
/// # Example
///
/// ```ignore
/// use hlight::theme::load_theme_set;
///
/// const THEMES: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/assets/set/theme.packdump"
/// ));
///
/// let set = load_theme_set(Some(THEMES));
/// ```
pub fn load_theme_set(set: Option<&[u8]>) -> ThemeSet {
  let msg = READ_DUMP_DATA_ERR;

  match set {
    Some(x) => dumps::from_uncompressed_data(x).expect(msg),
    #[cfg(feature = "preset-theme-set")]
    _ => dumps::from_uncompressed_data(set.unwrap_or(hlight_assets::THEME_SET))
      .expect(msg),
    #[cfg(not(feature = "preset-theme-set"))]
    _ => ThemeSet::default(),
  }
}

impl HighlightResource<'_> {
  /// Gets the theme or initialize it if it is not already set.
  pub fn get_or_init_theme(&self) -> &Theme {
    self
      .get_theme()
      .get_or_init(|| {
        let name = self.get_theme_name().as_str();
        let set = self.get_theme_set();
        &set.themes[name]
      })
  }

  /// This is the default theme set.
  ///
  /// # Example
  ///
  /// ```
  /// use hlight::HighlightResource;
  ///
  /// let set = HighlightResource::static_theme_set();
  /// let themes = &set.themes;
  ///
  /// for t in themes.keys() {
  ///     println!("{t}")
  /// }
  /// ```
  pub fn static_theme_set() -> &'static ThemeSet {
    static S: LazyLock<ThemeSet> = LazyLock::new(|| load_theme_set(None));
    &S
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn get_theme_set() {
    let themes = &HighlightResource::static_theme_set().themes;
    for t in themes.keys() {
      println!("{t}")
    }
  }
}
