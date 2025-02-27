#[ignore]
#[test]
fn test_output() {
  use hlight::{GenSyntax, HighLightRes, theme::names::ayu_dark};

  let s: &str = r#"
  [main]
  enabled = false
  "😎" = "🍥"
  float = nan
  "#;

  let res = HighLightRes::default()
    .with_background(false)
    .with_name(ayu_dark());

  let _ = GenSyntax::default()
    .with_dst_fmt("toml")
    .with_contents(s)
    .with_style(res.into())
    .run();

  // gen_syntax_highlight("toml", s, Some(&res), None)
  //   .expect("Failed to get highlighted toml text");
}

#[ignore]
#[test]
fn test_output_to_file() {
  use std::fs::File;

  use hlight::{GenSyntax, HighLightRes, theme::names::ayu_dark};

  let s: &str = r#"
  [main]
  enabled = false
  "😎" = "🍥"
  float = nan
  "#;

  let res = HighLightRes::default()
    .with_background(false)
    .with_name(ayu_dark());

  let mut file = File::create("tmp.txt").expect("Failed to create test.txt");

  let _ = GenSyntax::default()
    .with_dst_fmt("toml")
    .with_contents(s)
    .with_style(res.into())
    .with_writer(Some(&mut file))
    .run();

  // gen_syntax_highlight("toml", s, Some(&res), None)
  //   .expect("Failed to get highlighted toml text");
}
