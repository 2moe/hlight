use std::io;

use tap::Pipe;
use testutils::os_cmd::{CommandRepr, Runner, presets::CargoFmt};

#[test]
#[ignore]
fn fmt() -> io::Result<()> {
  CargoFmt::default()
    .pipe(CommandRepr::from)
    .pipe(Runner::from)
    .run()
}
