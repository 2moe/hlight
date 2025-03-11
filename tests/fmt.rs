use std::io;

use testutils::os_cmd::{RunnableCommand, presets::CargoFmt};

#[test]
#[ignore]
fn fmt() -> io::Result<()> {
  CargoFmt::default().run()
}
