// --theme-dir
// --syntax-dir
// --syntax-exclude-newline
// --compress
// --to /tmp/theme.packdump
use clap::{ColorChoice, Parser};
use getset::Getters;
use std::path::PathBuf;

pub(crate) const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const fn get_default_dir_by_os() -> &'static str {
    match () {
        #[cfg(windows)]
        () => "C:\\path\\to\\dir",
        #[cfg(not(windows))]
        () => "/path/to/dir",
    }
}

const fn get_default_fpath_by_os() -> &'static str {
    match () {
        #[cfg(windows)]
        () => "C:\\path\\to\\file-or-dir",
        #[cfg(not(windows))]
        () => "/path/to/file-or-dir",
    }
}
/// To dump Sublime's theme/syntax files as binary data.
///
/// Example: `hlight-dump --theme-dir assets/theme`
#[derive(Parser, Debug, Getters)]
#[getset(get = "pub(crate) with_prefix")]
#[command(arg_required_else_help = true)]
#[command(color = ColorChoice::Always)]
pub(crate) struct Cli {
    /// The directory where the specified theme file(s) are located
    #[arg(
        short = 'd',
        long,
        value_name = get_default_dir_by_os(),
        group = "src-dir",
        visible_alias = "td",
        value_hint = clap::ValueHint::DirPath,
        help_heading = "Src",
        // help = get_args_text("theme-dir"),
        // long_help = get_args_md("theme-dir-help"),
    )]
    theme_dir: Option<PathBuf>,

    /// Syntax file(s) directory
    #[arg(
        long,
        value_name = get_default_dir_by_os(),
        group = "src-dir",
        visible_alias = "sd",
        value_hint = clap::ValueHint::DirPath,
        help_heading = "Src",
    )]
    syntax_dir: Option<PathBuf>,

    /// Not including the newline character `\n`
    ///
    /// See also: https://docs.rs/syntect/latest/syntect/parsing/struct.SyntaxSetBuilder.html#method.add_from_folder
    #[arg(
        // 
        long,
        visible_alias = "ex-n",
        requires = "syntax_dir",
        help_heading = "Cfg",
    )]
    syntax_exclude_newline: bool,

    /// Compress the dumped data
    ///
    /// It will result in slower loading speed.
    #[arg(
        // 
        long,
        help_heading = "Cfg",
    )]
    compress: bool,

    /// Manually specifying the directory or filename for the dumped file.
    #[arg(
        short,
        value_name = get_default_fpath_by_os(),
        long,
        requires = "src-dir",
        value_hint = clap::ValueHint::AnyPath,
        help_heading = "Dst",
    )]
    to: Option<PathBuf>,

    /// Used for generating shell completion scripts
    #[arg(
        long,
        value_parser = ["zsh", "fish", "pwsh", "powershell", "bash", "elvish"],
        visible_alias = "sh-comp",
        value_name = "shell-name",
        help_heading = "Shell",
        // help = get_args_text("shell-completion"),
        // long_help = get_text("shell-completion-help"),
    )]
    shell_completion: Option<String>,

    /// Save the shell completion script to a specified directory or file
    #[arg(
        value_name = get_default_fpath_by_os(),
        long,
        requires = "shell_completion",
        value_hint = clap::ValueHint::AnyPath,
        help_heading = "Shell",
    )]
    save_sh_comp_to: Option<PathBuf>,

    /// Print the version of the tool.
    #[arg(long, short = 'V', help = PKG_VERSION)]
    version: bool,
}
