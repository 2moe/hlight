use crate::opt::parser::parse_args;
use anyhow::Context;
use env_logger::Env;

mod dump;
mod opt;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::new().filter_or("HLIGHT_LOG", "info")).init();
    log::debug!("Logger initialization completed.");

    log::debug!("Parsing command-line arguments.");
    parse_args().context("Failed to parse cli args")?;

    Ok(())
}
