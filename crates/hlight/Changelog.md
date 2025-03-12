# ChangeLog

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

- `HighLightRes` -> `HighlightResource`
- `get_theme_or_init_once()` -> `get_or_init_theme()`
- `Highlighter.resource::default()`: `Some(_)` -> `None`
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

- `HighLightRes.name` -> `HighLightRes.theme_name`
- `Highlighter.dst_fmt` -> `Highlighter.syntax_name`

## 0.0.4

- `GenSyntax` -> `Highlighter`

## 0.0.3

- remove `match_static_syntax()`
- `HighLightRes::syntax_set`: `'static` -> `'theme`

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
