use std::io::{self, BufWriter, Write};

use getset::{Getters, WithSetters};
use syntect::{
  easy::HighlightLines,
  parsing::SyntaxSet,
  util::{LinesWithEndings, as_24_bit_terminal_escaped},
};
use tap::Pipe;

use crate::{resource::HighLightRes, syntax::match_static_syntax};

#[derive(Getters, WithSetters)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct GenSyntax<'a> {
  dst_fmt: &'a str,
  contents: &'a str,
  style: Option<HighLightRes<'a>>,
  writer: Option<&'a mut dyn Write>,
}

impl Default for GenSyntax<'_> {
  fn default() -> Self {
    Self {
      dst_fmt: "markdown",
      contents: "",
      style: Some(HighLightRes::default()),
      writer: None,
    }
  }
}

impl GenSyntax<'_> {
  /// Prints syntax-highlighted code to either standard output or a provided
  /// writer, using the selected syntax highlighting style to highlight the code
  /// beforehand.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use hlight::HighLightRes;
  /// use hlight::GenSyntax;
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
  /// GenSyntax::default()
  ///   .with_dst_fmt("toml")
  ///   .with_contents(s)
  ///   .with_style(res.into())
  ///   .with_writer(Some(&mut file))
  ///   .run();
  /// ```
  pub fn run(self) -> io::Result<()> {
    let Self {
      dst_fmt,
      contents,
      style,
      writer,
    } = self;

    let mut stdout = std::io::stdout().pipe(BufWriter::new);

    let out = match writer {
      Some(w) => {
        drop(stdout);
        w
      }
      _ => &mut stdout as &mut dyn Write,
    };

    let hl_res = match style {
      Some(s)
        if !s
          .get_name()
          .eq_ignore_ascii_case("none") =>
      {
        s
      }
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

    let lines = HighlightLines::new(syntax, hl_res.get_theme_or_init_once());

    write_highlight_line(
      contents,
      lines,
      syntax_set,
      *hl_res.get_background(),
      out,
    )?;
    out.flush()?;

    log::debug!("Output complete");
    Ok(())
  }
}

/// Performs the actual highlighting of lines of code, and writes the
/// highlighted output to the specified output stream.
///
/// The function loops through each line of the `contents` parameter, uses the
/// `highlight_line` method to highlight each line, and gets the escaped 24-bit
/// terminal format of the highlighted ranges using the
/// `as_24_bit_terminal_escaped` function.
///
/// Finally, it writes the escaped 24-bit terminal format to the output.
fn write_highlight_line(
  contents: &str,
  mut highlight_lines: HighlightLines,
  syntax_set: &SyntaxSet,
  background: bool,
  out: &mut dyn Write,
) -> io::Result<()> {
  for line in LinesWithEndings::from(contents) {
    let ranges = highlight_lines
      .highlight_line(line, syntax_set)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let escaped = as_24_bit_terminal_escaped(&ranges[..], background);
    out.write_all(escaped.as_bytes())?
  }
  out.write_all(b"\x1B[0m")?;
  Ok(())
}

#[cfg(test)]
mod tests {
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
    // gen_syntax_highlight("toml", S, Some(&res), None)
    Ok(())
  }

  #[test]
  #[ignore]
  #[cfg(unix)]
  fn write_to_file() -> io::Result<()> {
    use std::fs::File;

    let res = HighLightRes::default().with_background(false);
    let mut file = File::create("/tmp/test.txt")?;
    // gen_syntax_highlight("toml", S, Some(&res), Some(&mut file))
    Ok(())
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
    // gen_syntax_highlight("pwsh", s, Some(&res), None)
    Ok(())
  }

  #[test]
  fn zsh_high_light() -> io::Result<()> {
    let s = r#"
        #compdef tomlyre

        autoload -U is-at-least

        _tomlyre() {
            typeset -A opt_args
            typeset -a _arguments_options
            local ret=1

            if is-at-least 5.2; then
                _arguments_options=(-s -S -C)
            else
                _arguments_options=(-s -C)
            fi

            local context curcontext="$curcontext" state line
            _arguments "${_arguments_options[@]}" \
        "#;

    let res = HighLightRes::default().with_background(true);
    // gen_syntax_highlight("pwsh", s, Some(&res), None)
    Ok(())
  }
}
