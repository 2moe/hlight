use crate::{resource::HighLightRes, theme::READ_DUMP_DATA_ERR};
use once_cell::sync::OnceCell;

#[cfg(feature = "preset-syntax-set")]
use syntect::dumps;

pub use syntect::parsing::{SyntaxReference, SyntaxSet};

type OnceSyntax = OnceCell<&'static SyntaxReference>;

#[cfg(feature = "preset-syntax-set")]
const SUBLIME_SYNTAXES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/theme-syntax-set/syntax-set.packdump"
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
///     "/assets/syntax-set.packdump"
/// ));
///
/// let set = load_syntax_set(Some(SYNTAXES));
/// ```
pub fn load_syntax_set(set: Option<&[u8]>) -> SyntaxSet {
    let msg = READ_DUMP_DATA_ERR;

    match set {
        Some(x) => dumps::from_uncompressed_data(x).expect(msg),
        #[cfg(feature = "preset-syntax-set")]
        _ => dumps::from_uncompressed_data(set.unwrap_or(SUBLIME_SYNTAXES))
            .expect(msg),
        #[allow(unreachable_patterns)]
        _ => SyntaxSet::default(),
    }
}

impl<'name> HighLightRes<'name> {
    /// This is the default syntax set.
    pub fn static_syntax_set() -> &'static SyntaxSet {
        static S: OnceCell<SyntaxSet> = OnceCell::new();
        S.get_or_init(|| load_syntax_set(None))
    }
}

/// It matches the format string against known syntax formats(e.g. md, toml, json, yaml), returning a reference to the corresponding syntax if found.
///
/// If the format string does not match any known formats, it uses a generic function to find a syntax matching the format string.
///
/// # Example
///
/// ```
/// use hlight::syntax::match_static_syntax;
/// use hlight::HighLightRes;
///
/// let set = HighLightRes::static_syntax_set();
/// let syntax = match_static_syntax(set, "toml");
/// ```
pub fn match_static_syntax(
    set: &'static SyntaxSet,
    fmt: &str,
) -> &'static SyntaxReference {
    match fmt {
        "md" | "markdown" => get_markdown(set),
        "toml" => get_toml(set),
        "yaml" | "yml" => get_yaml(set),
        "json" | "json5" | "ron" => get_json(set),
        "pwsh" | "ps1" | "powershell" => get_pwsh(set),
        _ => find_syntax(set, fmt),
    }
}

/// Finds and returns the appropriate syntax highlighting definition from a `SyntaxSet` based on a given destination format. If not found, it will fallback to json.
pub(crate) fn find_syntax<'a>(
    syntax_set: &'a SyntaxSet,
    dst_fmt: &str,
) -> &'a SyntaxReference {
    syntax_set
        .find_syntax_by_name(dst_fmt)
        .unwrap_or_else(|| {
            let to_json = || syntax_set.find_syntax_by_extension("json");
            match dst_fmt {
                "sexp" | "lexpr" => syntax_set
                    .find_syntax_by_extension("lisp")
                    .or_else(to_json),
                _ => to_json(),
            }
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
        })
}

/// Finds syntax reference by name.
///
/// It takes a syntax set and a name as input parameters. It tries to find the syntax reference for the given name in the syntax set. If it finds the syntax reference, it returns it. If it doesn't find the syntax reference by name, it tries to find it by extension. If it still doesn't find the syntax reference, it returns the plain text syntax reference.
///
/// # Example
///
/// ```
/// use hlight::{syntax::find_syntax_name, HighLightRes};
///
/// let set = HighLightRes::static_syntax_set();
/// let syntax = find_syntax_name(set, "Markdown");
/// ```
pub fn find_syntax_by_name<'a>(
    set: &'a SyntaxSet,
    name: &str,
) -> &'a SyntaxReference {
    set.find_syntax_by_name(name)
        .unwrap_or_else(|| {
            set.find_syntax_by_extension(name)
                .unwrap_or(set.find_syntax_plain_text())
        })
}

// static SyntaxReference:
// md, json, yaml, toml
//
fn get_markdown(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_by_name(set, "Markdown"))
}

fn get_json(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_by_name(set, "JSON"))
}

fn get_yaml(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_by_name(set, "YAML"))
}

fn get_toml(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_by_name(set, "TOML"))
}

fn get_pwsh(set: &'static SyntaxSet) -> &'static SyntaxReference {
    static S: OnceSyntax = OnceCell::new();
    S.get_or_init(|| find_syntax_by_name(set, "PowerShell"))
}
