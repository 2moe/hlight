# hlight

A library for output syntax highlighting.

## ChangeLog

[Changelog.md](Changelog.md)

## Quick Start

### add dependency

```sh
cargo add hlight
```

### print to stdout

```rust
use hlight::{Highlighter, theme::names::ayu_dark, HighlightResource};


let s: &str = r#"
[main]
enabled = false
"😎" = "🍥"
float = nan
"#;

let res = HighlightResource::default()
  .with_background(false)
  .with_theme_name(ayu_dark());

let _ = Highlighter::default()
  .with_syntax_name("toml")
  .with_content(s)
  .with_resource(Some(&res))
  .run();
```

output:

![toml.svg](assets/svg/toml.svg)

### write to file

```rust
use std::fs::File;

use hlight::{Highlighter, HighlightResource, theme::names::ayu_dark};

let s: &str = r#"
[main]
enabled = false
"😎" = "🍥"
float = nan
"#;

let res = HighlightResource::default()
  .with_background(false)
  .with_theme_name(ayu_dark());

let mut file = File::create("tmp.txt").expect("Failed to create test.txt");

let _ = Highlighter::default()
  .with_syntax_name("toml")
  .with_content(s)
  .with_resource(Some(&res))
  .with_writer(Some(&mut file))
  .run();
```

## Advanced

### Load custom set

> The `["preset-syntax-set", "preset-theme-set"]` features are enabled by default. If you want to customize the set, you don't need to enable these features.

disable default features:

```sh
cargo add hlight --no-default-features
```

#### theme-set

```rust
use hlight::{
    theme::{load_theme_set, ThemeSet},
    HighlightResource,
};
use std::borrow::Cow;

const THEMES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/set/theme.packdump"
));

fn main() {
    let set = load_theme_set(Some(THEMES));

    let res = HighlightResource::default().with_theme_set(&set).with_theme_name("Custom-theme-name".into());

    show_theme_set(res.get_theme_set())
}

fn show_theme_set(set: &ThemeSet) {
    for k in set.themes.keys() {
        println!("{k}")
    }
}
```

#### syntax-set

```rust
use hlight::{
    syntax::{load_syntax_set, SyntaxSet},
    HighlightResource,
};
use std::sync::OnceLock;

const SYNTAXES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/set/syntax.packdump"
));

fn static_syntax_set() -> &'static SyntaxSet {
    static S: OnceLock<SyntaxSet> = OnceLock::new();
    S.get_or_init(|| load_syntax_set(Some(SYNTAXES)))
}

fn main() {
    let res = HighlightResource::default().with_syntax_set(static_syntax_set());
    show_syntax_set(res.get_syntax_set())
}

fn show_syntax_set(set: &SyntaxSet) {
    for (name, ext) in set
        .syntaxes()
        .iter()
        .map(|x| (&x.name, &x.file_extensions))
    {
        println!(
            "name: {name}\n\
            ext: {ext:?}\n---"
        )
    }
}
```
