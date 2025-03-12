//! ```ignore, sh
//! cargo open-doc
//! ```
use std::io;

use testutils::os_cmd::{RunnableCommand, presets::CargoDoc};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  for pkg in [
    // "hlight",
    // "hlight-dump",
    "hlight-assets",
  ] {
    CargoDoc::default()
      .with_pkg(pkg)
      .with_enable_private_items(false)
      .run()?
  }
  Ok(())
}
