use crate::{resource::HighLightRes, syntax::match_static_syntax};
use std::io::{self, BufWriter, Write};
use syntect::{
    easy::HighlightLines,
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

/// Prints syntax-highlighted code to either standard output or a provided writer, using the selected syntax highlighting style to highlight the code beforehand.
///
/// # Example
///
/// ```no_run
/// use hlight::HighLightRes;
/// use hlight::gen_syntax_highlight;
/// use std::fs::File;
///
/// let s = "
/// [main]
/// enabled = false
/// float = 314e-2
/// ";
///
/// let res = HighLightRes::default().with_background(false);
/// let mut file = File::create("test.txt").expect("Failed to create test.txt");
///
/// gen_syntax_highlight("toml", s, Some(&res), Some(&mut file)).expect("Failed to get syntax highlighting");
/// ```
pub fn gen_syntax_highlight(
    dst_fmt: &str,
    contents: &str,
    style: Option<&HighLightRes>,
    writer: Option<&mut dyn Write>,
) -> io::Result<()> {
    let mut stdout = BufWriter::new(std::io::stdout());

    let out = match writer {
        Some(w) => {
            drop(stdout);
            w
        }
        _ => &mut stdout as &mut dyn Write,
    };

    let hl_res = match style {
        Some(s) if s.get_name() != "None" => s,
        _ => {
            out.write_all(contents.as_bytes())?;
            return out.flush();
        }
    };
    let syntax_set = hl_res.get_syntax_set();

    log::debug!("About to Load the SyntaxSet and ThemeSet");

    let syntax = match_static_syntax(syntax_set, dst_fmt);

    log::trace!("ext: {:?}", syntax.file_extensions);
    log::debug!("syntax:{}", syntax.name);

    let highlight = HighlightLines::new(syntax, hl_res.set_theme_once());

    write_highlight_line(
        contents,
        highlight,
        syntax_set,
        *hl_res.get_background(),
        out,
    )?;
    out.flush()?;

    log::debug!("Output complete");
    Ok(())
}

/// Performs the actual highlighting of lines of code, and writes the highlighted output to the specified output stream.
///
/// The function loops through each line of the `contents` parameter, uses the `highlight_line` method to highlight each line, and gets the escaped 24-bit terminal format of the highlighted ranges using the `as_24_bit_terminal_escaped` function.
///
/// Finally, it writes the escaped 24-bit terminal format to the output.
fn write_highlight_line(
    contents: &str,
    mut highlight: HighlightLines,
    syntax_set: &SyntaxSet,
    background: bool,
    out: &mut dyn Write,
) -> io::Result<()> {
    for line in LinesWithEndings::from(contents) {
        let ranges = highlight
            .highlight_line(line, syntax_set)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            })?;

        let escaped = as_24_bit_terminal_escaped(&ranges[..], background);
        out.write_all(escaped.as_bytes())?
    }
    out.write_all(b"\x1B[0m")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    const S: &str = r#"
    [main]
    enabled = false
    "😎" = "🍥"
    float = nan
    "#;

    #[test]
    fn print_highlighted_text() -> io::Result<()> {
        let res = HighLightRes::default();
        gen_syntax_highlight("toml", S, Some(&res), None)
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn write_to_file() -> io::Result<()> {
        let res = HighLightRes::default().with_background(false);
        let mut file = File::create("/tmp/test.txt")?;
        gen_syntax_highlight("toml", S, Some(&res), Some(&mut file))
    }

    #[test]
    fn get_pwsh() -> io::Result<()> {
        let s = r#"
        using namespace System

        $dir = [IO.Path]::GetDirectoryName($profile)
        
        if (! [IO.Directory]::Exists($dir)) {
            [Console]::Write("Creating Directory: ")
            Write-Host "$dir" -ForegroundColor Cyan
            [IO.Directory]::CreateDirectory($dir) | Out-Null
        }
        $file = Join-Path $dir "_tomlyre.ps1"
        
        [Console]::OutputEncoding = [Text.Encoding]::UTF8
        # tomlyre --sh-comp pwsh | Out-File -FilePath $file -Encoding utf8
        tomlyre --sh-comp pwsh > $file
        
        . $file
        # [IO.File]::AppendAllText($profile, "`nInvoke-Expression '$file'`n")
        "`nInvoke-Expression '$file'" >> $profile
        "#;

        let res = HighLightRes::default().with_background(false);
        gen_syntax_highlight("pwsh", s, Some(&res), None)
    }
}
