use std::{borrow::Cow, path::Path, process::exit};

use crate::{
    dump::Dump,
    opt::{
        args::{Cli, PKG_VERSION},
        completion::get_shell_completion,
        dir::SrcDir,
    },
};
use clap::Parser;

pub(crate) fn parse_args() -> anyhow::Result<()> {
    let args = Cli::parse();

    if get_shell_completion(&args)? {
        exit(0)
    };

    if *args.get_version() {
        return Ok(println!("{}", PKG_VERSION));
    }

    let compression = *args.get_compress();
    let ex_newline = *args.get_syntax_exclude_newline();

    let (src, fname) = match (args.get_syntax_dir(), args.get_theme_dir()) {
        (Some(p), _) => (SrcDir::Syntax(p), "syntax-set.packdump"),
        (_, Some(p)) => (SrcDir::Theme(p), "theme-set.packdump"),
        _ => panic!("You need to pass in Theme or Syntax Dir"),
    };

    let dst = match args.get_to() {
        Some(p) if p.is_dir() => Cow::from(p.join(fname)),
        Some(p) => Cow::from(p),
        _ => Cow::from(Path::new(fname)),
    };

    let dump = Dump::new(compression, src, dst, ex_newline);
    dump.dump_set()?;

    Ok(())
}
