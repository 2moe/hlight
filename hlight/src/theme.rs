use std::borrow::Cow;

use once_cell::sync::OnceCell;
pub use syntect::{
    dumps,
    highlighting::{Theme, ThemeSet},
};

use crate::resource::HighLightRes;

#[cfg(feature = "preset-theme-set")]
const THEME_SET: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/theme-syntax-set/theme-set.packdump"
));

pub const READ_DUMP_DATA_ERR: &str = "Failed to read dump data";

/// Loads a set of themes.
///
/// If the parameter is None, the default theme set is used.
///
/// # Example
///
/// ```no_run
/// use hlight::theme::load_theme_set;
///
/// const THEMES: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/assets/theme-set.packdump"
/// ));
///
/// let set = load_theme_set(Some(THEMES));
/// ```
pub fn load_theme_set(set: Option<&[u8]>) -> ThemeSet {
    let msg = READ_DUMP_DATA_ERR;

    match set {
        Some(x) => dumps::from_uncompressed_data(x).expect(msg),
        #[cfg(feature = "preset-theme-set")]
        _ => dumps::from_uncompressed_data(set.unwrap_or(THEME_SET)).expect(msg),
        #[allow(unreachable_patterns)]
        _ => ThemeSet::default(),
    }
}

impl<'name> HighLightRes<'name> {
    /// Gets the theme or initialize it if it is not already set.
    pub fn set_theme_once(&self) -> &Theme {
        self.get_theme()
            .get_or_init(|| {
                let name = self.get_name().as_ref();
                let set = self.get_theme_set();
                set.themes[name].to_owned()
            })
    }

    /// This is the default theme set.
    ///
    /// # Example
    ///
    /// ```
    /// use hlight::HighLightRes;
    ///
    /// let set = HighLightRes::static_theme_set();
    /// let themes = &set.themes;
    ///
    /// for t in themes.keys() {
    ///     println!("{t}")
    /// }
    /// ```
    pub fn static_theme_set() -> &'static ThemeSet {
        static S: OnceCell<ThemeSet> = OnceCell::new();
        S.get_or_init(|| load_theme_set(None))
    }
}

const fn monokai_theme_name() -> &'static str {
    "Monokai Extended"
}

/// "Monokai Extended"
pub fn theme_monokai<'a>() -> Cow<'a, str> {
    Cow::from(monokai_theme_name())
}

/// "ayu-dark"
pub fn theme_ayu_dark<'a>() -> Cow<'a, str> {
    Cow::from(ayu_dark_theme_name())
}

const fn ayu_dark_theme_name() -> &'static str {
    "ayu-dark"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_theme_set() {
        let themes = &HighLightRes::static_theme_set().themes;
        for t in themes.keys() {
            println!("{t}")
        }
    }
}
