#[ignore]
#[test]
fn test_output() {
  use hlight::{HighlightResource, Highlighter, theme::names::ayu_dark};

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
    .with_resource((&res).into())
    .run();

  // gen_syntax_highlight("toml", s, Some(&res), None)
  //   .expect("Failed to get highlighted toml text");
}

#[ignore]
#[test]
fn test_output_to_file() {
  use std::fs::File;

  use hlight::{HighlightResource, Highlighter, theme::names::ayu_dark};

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
    .with_resource((&res).into())
    .with_writer(Some(&mut file))
    .run();

  // gen_syntax_highlight("toml", s, Some(&res), None)
  //   .expect("Failed to get highlighted toml text");
}
