use crate::{dump::create_parent_dir, opt::args::Cli};
use clap::CommandFactory;
use clap_complete::{generate, shells, Generator};
use log::{debug, info};
use std::{
    borrow::Cow,
    env,
    fs::File,
    io::{self, Write},
};

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

/// Generates shell completion scripts using clap_complete.
/// It takes a generic type parameter G which specifies the type of shell to generate the script for, and returns a byte vector containing the generated script.
fn gen_completion<G: Generator>(g: G) -> Vec<u8> {
    let mut cmd = Cli::command();
    let mut s = Vec::with_capacity(1024 * 13);
    generate(g, &mut cmd, BIN_NAME, &mut s);
    s
}

/// Generates a shell completion script based on the specified shell type in args, and prints it out.
pub(crate) fn get_shell_completion(args: &Cli) -> io::Result<bool> {
    let Some(sh) = args.get_shell_completion() else { return Ok(false) };
    let save_to_file = args.get_save_sh_comp_to();

    debug!("Getting the shell completionscript...");

    fn gen<G: Generator>(g: G) -> Vec<u8> {
        gen_completion(g)
    }

    let sh_name = sh.to_ascii_lowercase();

    let v = {
        use shells::*;
        match sh_name.as_str() {
            "bash" => gen(Bash),
            "zsh" => gen(Zsh),
            "pwsh" | "powershell" => gen(PowerShell),
            "fish" => gen(Fish),
            "elvish" => gen(Elvish),
            _ => {
                info!(
                    "system shell: {:?}",
                    env::var("SHELL").unwrap_or_else(|_| "Unknown".to_owned())
                );
                panic!("Unsupported shell: {}", sh_name)
            }
        }
    };

    let dst = match save_to_file {
        Some(p) if p.is_dir() => Cow::from(p.join(format!("_{}", BIN_NAME))),
        Some(p) => Cow::from(p),
        _ => {
            let s = String::from_utf8_lossy(&v);
            println!("{s}");
            return Ok(true);
        }
    };

    create_parent_dir(&dst)?;

    info!("Writing to: {}", dst.display());
    let mut file = File::create(dst)?;

    #[cfg(windows)]
    {
        const U8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
        file.write_all(&U8_BOM)?;
    }

    file.write_all(&v)?;

    info!("Written to file");
    Ok(true)
}
