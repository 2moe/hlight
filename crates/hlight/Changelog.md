# ChangeLog

## 0.0.11

- remove `HighlightResource::new()`
- `HighlightResource::theme_set`: `&'theme ThemeSet` => `HlightThemeSet<'theme>`
- `HighlightResource::syntax_set`: `&'theme SyntaxSet` => `Cow<'theme, SyntaxSet>`
- `Highlighter`: +`prefer_syntax_ext`

## 0.0.10

- add `to_ansi_256color()`
- add a `true_color` field to the `Highlighter` struct.
  - When set to `false`, outputs highlighted text in 256-color mode instead of 24-bit true color.

## 0.0.9

- chore(static-set): OnceLock => LazyLock
- `HighlightResource::new()`: `name: &str` => `name: impl Into<CmString>`
- fix `Highlighter.writer`: lifetime => `'w`

Previous:

```rust
pub struct Highlighter<'a> {
  writer: Option<&'a mut dyn Write>,
  ...
}
```

Now:

```rust
pub struct Highlighter<'a, 'w> {
  writer: Option<&'w mut dyn Write>,
  ...
}
```

## 0.0.8

Separate the theme-set and syntax-set into an individual crate: hlight-assets.

## 0.0.7

- change the type of `HighlightResource.theme`

Previous:

```rust
theme: OnceLock<Theme>,
```

Now:

```rust
theme: OnceLock<&'theme Theme>,
```

## 0.0.6

Breaking changes:

- `HighLightRes` => `HighlightResource`
- `get_theme_or_init_once()` => `get_or_init_theme()`
- `Highlighter.resource::default()`: `Some(_)` => `None`
- change the `Highlighter.resource` type from `Option<_>` to `Option<&_>`

Previous:

```rust
  resource: Option<HighlightResource<'a>>,
```

Now:

```rust
  resource: Option<&'a HighlightResource<'a>>,
```

## 0.0.5

Breaking changes:

- `HighLightRes.name` => `HighLightRes.theme_name`
- `Highlighter.dst_fmt` => `Highlighter.syntax_name`

## 0.0.4

- `GenSyntax` => `Highlighter`

## 0.0.3

- remove `match_static_syntax()`
- `HighLightRes::syntax_set`: `'static` => `'theme`

## 0.0.2

- Upgrade to Rust Edition 2024
- Minimum supported Rust version (MSRV): 1.85
- Change the `name` field type of the `HighLightRes` struct from `Cow<'theme, str>` to `compact_str::CompactString`

```rust
pub struct HighLightRes<'theme> {
  name: CmString,
  ...
}
```

- Remove `get_xx_mut` and use `with_xx()` instead

Previously setting the theme name:

```rust
let mut res = HighLightRes::default();
let theme_name = hlight::theme::theme_ayu_dark();

*res.get_name_mut() = theme_name;
```

Now:

```rust
let theme_name = hlight::theme::names::ayu_dark();

let res = HighLightRes::default().with_name(theme_name);
```
