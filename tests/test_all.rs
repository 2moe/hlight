use std::io;

use testutils::os_cmd::{
  RunnableCommand,
  presets::{
    CargoCmd,
    cargo_build::{CargoProfile, SubCmd},
  },
};

fn new_cargo_test_builder() -> CargoCmd {
  CargoCmd::default()
    .with_sub_command(SubCmd::Test)
    .with_profile(CargoProfile::Debug)
    .with_all_packages(true)
}

#[ignore]
#[test]
fn test_all() -> io::Result<()> {
  new_cargo_test_builder()
    .with_all_features(true)
    .run()?;

  new_cargo_test_builder()
    .with_all_features(false)
    .run()?;

  Ok(())
}
