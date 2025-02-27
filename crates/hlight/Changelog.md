# ChangeLog

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
