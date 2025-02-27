use std::path::Path;

#[derive(Debug)]
pub(crate) enum SrcDir<'p> {
  Theme(&'p Path),
  Syntax(&'p Path),
}

impl Default for SrcDir<'_> {
  fn default() -> Self {
    Self::Theme(Path::new(""))
  }
}
