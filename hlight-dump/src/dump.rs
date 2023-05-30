use anyhow::Context;
use getset::Getters;
use log::{debug, info};
use serde::Serialize;
use std::{borrow::Cow, fs, io, path::Path};
use syntect::{
    dumps::{self, dump_to_uncompressed_file},
    highlighting::ThemeSet,
    parsing::SyntaxSetBuilder,
};

use crate::opt::dir::SrcDir;

#[derive(Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
pub(crate) struct Dump<'p> {
    compression: bool,
    src: SrcDir<'p>,
    dst: Cow<'p, Path>,
    exclude_newline: bool,
}

impl<'p> Dump<'p> {
    pub(crate) fn new(
        compression: bool,
        src: SrcDir<'p>,
        dst: Cow<'p, Path>,
        exclude_newline: bool,
    ) -> Self {
        Self {
            compression,
            src,
            dst,
            exclude_newline,
        }
    }

    pub(crate) fn dump_set(&self) -> anyhow::Result<()> {
        let comp = self.get_compression();
        let dst = self.get_dst();

        info!("dst: {dst:?}");

        match self.get_src() {
            SrcDir::Syntax(p) => {
                let mut builder = SyntaxSetBuilder::default();
                builder
                    .add_from_folder(p, !self.get_exclude_newline())
                    .context("Failed to add syntax set from dir")?;

                let set = builder.build();
                dump_to_file(*comp, &set, dst)?;
            }
            SrcDir::Theme(p) => {
                let set = ThemeSet::load_from_folder(p)?;
                dump_to_file(*comp, &set, dst)?;
            }
        }
        Ok(())
    }
}

pub(crate) fn create_parent_dir(dst: &Path) -> io::Result<()> {
    match dst.parent() {
        Some(p) if !p.exists() => {
            info!("Creating dir: {p:?}");
            fs::create_dir_all(p)
        }
        _ => Ok(()),
    }
}

fn dump_to_file<T: Serialize>(
    compression: bool,
    set: &T,
    dst: &Path,
) -> anyhow::Result<()> {
    create_parent_dir(dst)?;

    if compression {
        dumps::dump_to_file(set, dst)?;
    } else {
        dump_to_uncompressed_file(set, dst)?;
    }
    debug!("Dump completed!");
    Ok(())
}
