use std::{cell::RefCell, collections::BTreeMap, io};

use hlight::{HighlightResource, syntax::SyntaxSet};
use rayon::prelude::*;
use syntect::{easy::HighlightLines, util::as_24_bit_terminal_escaped};
use tap::Pipe;
use testutils::simple_benchmark;
use thread_local::ThreadLocal;

#[test]
fn print() {
  let shared_syntax = HighlightResource::static_syntax_set();
  let shared_theme = HighlightResource::static_theme_set();
  let theme = &shared_theme.themes["ayu-dark"];

  let code = r#"
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
  .repeat(1000);

  let highlighted = parallel_highlight(&code, shared_syntax, theme);

  println!("{}", highlighted);
}

fn parallel_highlight(
  code: &str,
  syntax_set: &SyntaxSet,
  theme: &syntect::highlighting::Theme,
) -> String {
  let syntax = syntax_set
    .find_syntax_by_extension("toml")
    .unwrap();

  let tl_hl = ThreadLocal::new(); // 线程局部存储

  let processed: BTreeMap<_, _> = code
    .split_inclusive('\n')
    .enumerate()
    .par_bridge()
    .map(|(idx, line)| {
      // let mut hl = HighlightLines::new(syntax, theme);

      let hl_cell =
        tl_hl.get_or(|| HighlightLines::new(syntax, theme).pipe(RefCell::new));

      let mut hl = hl_cell.borrow_mut();

      let ranges = hl //  cannot borrow `*hl` as mutable, as it is behind a `&` reference
        .highlight_line(line, syntax_set)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        .unwrap();

      let highlighted = as_24_bit_terminal_escaped(&ranges[..], false);

      (idx, highlighted)
    })
    .collect();

  processed
    .values()
    .cloned()
    .collect()
}

/// - release: 226.011125ms
/// - debug: 226.53725ms
#[ignore]
#[test]
fn bench_test_parallel() {
  simple_benchmark(print);
  // println!("{}", highlighted);
}
