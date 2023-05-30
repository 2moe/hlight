# hlight

A library for output syntax highlighting.

## Quick Start

### add dep

```sh
cargo add hlight
```

### print to stdout

```rust
use hlight::{gen_syntax_highlight, theme::theme_ayu_dark, HighLightRes};


let s: &str = r#"
[main]
enabled = false
"😎" = "🍥"
float = nan
"#;

let mut res = HighLightRes::default().with_background(false);
// theme_ayu_dark: Cow::from("ayu-dark")
*res.get_name_mut() = theme_ayu_dark();

gen_syntax_highlight("toml", s, Some(&res), None)
    .expect("Failed to get highlighted toml text");
```

output:

![toml.svg](assets/svg/toml.svg)

### write to file

```rust
use std::fs::File;

let mut file = File::create("test.txt").expect("Failed to create test.txt");
gen_syntax_highlight("toml", s, Some(&res), Some(&mut file))
    .expect("Unable to write syntax-highlighted text to file.")
```

## Advanced

### Load custom set

> The `["preset-syntax-set", "preset-theme-set"]` features are enabled by default. If you want to customize the set, you don't need to load these features.

add deps:

```sh
cargo add hlight --no-default-features
cargo add once_cell
```

#### theme-set

```rust
use std::borrow::Cow;
use hlight::{theme::load_theme_set, HighLightRes};

fn main() {
    const THEMES: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/theme-set.packdump"
    ));
    let set = load_theme_set(Some(THEMES));

    let mut res = HighLightRes::default();

    *res.get_theme_set_mut() = &set;
    *res.get_name_mut() = Cow::from("Custom-theme-name");
}
```

#### syntax-set

```rust
use hlight::{
    syntax::{load_syntax_set, SyntaxSet},
    HighLightRes,
};
// use std::cell::OnceCell;
use once_cell::sync::OnceCell;

const SYNTAXES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/theme-syntax-set/syntax-set.packdump"
));

fn static_syntax_set() -> &'static SyntaxSet {
    static S: OnceCell<SyntaxSet> = OnceCell::new();
    S.get_or_init(|| load_syntax_set(Some(SYNTAXES)))
}

fn main() {
    let set = static_syntax_set();

    let mut res = HighLightRes::default();
    *res.get_syntax_set_mut() = set;
}
```
