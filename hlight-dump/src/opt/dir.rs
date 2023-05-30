use std::path::Path;

#[derive(Debug)]
pub(crate) enum SrcDir<'p> {
    Theme(&'p Path),
    Syntax(&'p Path),
}

impl<'p> Default for SrcDir<'p> {
    fn default() -> Self {
        Self::Theme(Path::new(""))
    }
}
