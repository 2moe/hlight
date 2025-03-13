use testutils::simple_benchmark;

#[test]
#[ignore]
fn test_output() -> std::io::Result<()> {
  use hlight::{HighlightResource, Highlighter, theme::names::ayu_dark};

  let s = r#"
  [main]
  enabled = false
  "😎" = "🍥"
  float = nan

  data = """
    a = 3
    b = 4
    c = true
  """
  "#
  .repeat(10);

  let res = HighlightResource::default()
    .with_background(true)
    .with_theme_name(ayu_dark());

  Highlighter::default()
    .with_syntax_name("toml")
    .with_content(&s)
    .with_resource((&res).into())
    .run()
}

#[ignore]
#[test]
fn bench_test_output() {
  simple_benchmark(test_output);
}

#[ignore]
#[test]
fn test_output_to_file() -> std::io::Result<()> {
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

  Highlighter::default()
    .with_syntax_name("toml")
    .with_content(s)
    .with_resource((&res).into())
    .with_writer(Some(&mut file))
    .run()
}
