//! ```ignore, sh
//! cargo open-doc
//! ```
use std::io;

use testutils::os_cmd::{RunnableCommand, presets::CargoDoc};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  let new_doc = |pkg| {
    CargoDoc::default()
      .with_pkg(pkg)
      .with_enable_private_items(false)
  };

  [
    "hlight",
    // "hlight-dump",
    // "hlight-assets",
  ]
  .into_iter()
  .map(new_doc)
  .try_for_each(RunnableCommand::run)
}
