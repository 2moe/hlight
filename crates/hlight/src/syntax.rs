use std::sync::OnceLock;

#[cfg(feature = "preset-syntax-set")]
use syntect::dumps;
pub use syntect::parsing::{SyntaxReference, SyntaxSet};

use crate::{resource::HighLightRes, theme::READ_DUMP_DATA_ERR};

#[cfg(feature = "preset-syntax-set")]
const SUBLIME_SYNTAXES: &[u8] = include_bytes!(concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/assets/set/syntax.packdump"
));

/// Loads a set of syntaxes.
///
/// If the parameter is None, the default syntax set is used.
///
/// # Example
///
/// ```no_run
/// use hlight::syntax::load_syntax_set;
///
/// const SYNTAXES: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/assets/set/syntax.packdump"
/// ));
///
/// let set = load_syntax_set(Some(SYNTAXES));
/// ```
pub fn load_syntax_set(set: Option<&[u8]>) -> SyntaxSet {
  let msg = READ_DUMP_DATA_ERR;

  match set {
    Some(x) => dumps::from_uncompressed_data(x).expect(msg),
    #[cfg(feature = "preset-syntax-set")]
    _ => dumps::from_uncompressed_data(set.unwrap_or(SUBLIME_SYNTAXES)).expect(msg),
    #[allow(unreachable_patterns)]
    _ => SyntaxSet::default(),
  }
}

impl HighLightRes<'_> {
  /// This is the default syntax set.
  ///
  /// # Example
  ///
  /// ```
  /// use hlight::HighLightRes;
  ///
  /// let set = HighLightRes::static_syntax_set();
  ///
  /// set.syntaxes()
  ///     .iter()
  ///     .map(|x| (&x.name, &x.file_extensions))
  ///     .for_each(|(name, ext)| {
  ///         println!(
  ///         "name: {name}\n\
  ///         ext:{ext:?}\n---"
  ///         )
  ///     });
  /// ```
  pub fn static_syntax_set() -> &'static SyntaxSet {
    static S: OnceLock<SyntaxSet> = OnceLock::new();
    S.get_or_init(|| load_syntax_set(None))
  }
}

// pub fn find_syntax<'a>(set: &'a SyntaxSet, fmt: &str) -> &'a SyntaxReference
// {   let find = |name| find_syntax_name(set, name);

//   match fmt {
//     "md" | "markdown" => find("Markdown"),
//     "toml" => find("TOML"),
//     "yaml" | "yml" => find("YAML"),
//     "json" | "json5" | "ron" => find("JSON"),
//     "pwsh" | "ps1" | "powershell" => find("PowerShell"),
//     _ => find_syntax(set, fmt),
//   }
// }

/// Finds and returns the appropriate syntax highlighting definition from a
/// `SyntaxSet` based on a given destination format. If not found, it will
/// fallback to json.
pub fn find_syntax<'a>(
  set: &'a SyntaxSet,
  syntax_name: &str,
) -> &'a SyntaxReference {
  set
    .find_syntax_by_extension(syntax_name)
    .unwrap_or_else(|| {
      set
        .find_syntax_by_name(syntax_name)
        .unwrap_or_else(|| {
          let to_json = || set.find_syntax_by_extension("json");

          match syntax_name {
            "sexp" | "lexpr" => set
              .find_syntax_by_extension("lisp")
              .or_else(to_json),
            _ => to_json(),
          }
          .unwrap_or_else(|| set.find_syntax_plain_text())
        })
    })
}

/// Finds syntax reference by name.
///
/// It takes a syntax set and a name as input parameters. It tries to find the
/// syntax reference for the given name in the syntax set. If it finds the
/// syntax reference, it returns it. If it doesn't find the syntax reference by
/// name, it tries to find it by extension. If it still doesn't find the syntax
/// reference, it returns the plain text syntax reference.
///
/// # Example
///
/// ```
/// use hlight::{syntax::find_syntax_name, HighLightRes};
///
/// let set = HighLightRes::static_syntax_set();
/// let syntax = find_syntax_name(set, "Markdown");
/// ```
pub fn find_syntax_name<'a>(set: &'a SyntaxSet, name: &str) -> &'a SyntaxReference {
  set
    .find_syntax_by_name(name)
    .unwrap_or_else(|| {
      set
        .find_syntax_by_extension(name)
        .unwrap_or(set.find_syntax_plain_text())
    })
}

#[cfg(test)]
mod tests {
  use crate::HighLightRes;

  #[test]
  #[ignore]
  fn iter_static_set() {
    let set = HighLightRes::static_syntax_set();

    set
      .syntaxes()
      .iter()
      .map(|x| (&x.name, &x.file_extensions))
      .for_each(|(name, ext)| {
        println!(
          "name: {name}\n\
                    ext:{ext:?}\n---"
        )
      });
  }
}
