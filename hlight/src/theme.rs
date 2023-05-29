use once_cell::sync::OnceCell;
pub use syntect::{
    dumps,
    highlighting::{Theme, ThemeSet},
};

use crate::resource::HighLightRes;

const THEME_SET: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/syntect/theme-set.packdump"
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
    dumps::from_uncompressed_data(set.unwrap_or(THEME_SET))
        .expect(READ_DUMP_DATA_ERR)
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
    pub fn static_theme_set() -> &'static ThemeSet {
        static S: OnceCell<ThemeSet> = OnceCell::new();
        S.get_or_init(|| load_theme_set(None))
    }
}

pub const fn monokai_theme_name() -> &'static str {
    "Monokai Extended"
}
pub const fn ayu_dark_theme_name() -> &'static str {
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
