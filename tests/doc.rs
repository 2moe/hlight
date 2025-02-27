//! ```ignore, sh
//! cargo open-doc
//! ```
use std::io;

use tap::Pipe;
use testutils::os_cmd::{Runner, presets::CargoDoc};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  for pkg in [
    "hlight",
    // "hlight-dump",
  ] {
    CargoDoc::default()
      .with_pkg(pkg)
      .pipe(Runner::from)
      .run()?
  }
  Ok(())
}
