use std::io::{self, BufWriter, Write};

use getset::{Getters, WithSetters};
use syntect::{
  easy::HighlightLines,
  parsing::SyntaxSet,
  util::{LinesWithEndings, as_24_bit_terminal_escaped},
};
use tap::Pipe;

use crate::{resource::HighlightResource, syntax::find_syntax};

#[derive(Getters, WithSetters)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct Highlighter<'a> {
  /// target syntax format (e.g., "json")
  syntax_name: &'a str,
  content: &'a str,
  resource: Option<&'a HighlightResource<'a>>,
  writer: Option<&'a mut dyn Write>,
}

impl Default for Highlighter<'_> {
  fn default() -> Self {
    Self {
      syntax_name: "markdown",
      content: "",
      resource: None, //Some(HighlightResource::default()),
      writer: None,
    }
  }
}

impl Highlighter<'_> {
  /// Prints syntax-highlighted code to either standard output or a provided
  /// writer, using the selected syntax highlighting style to highlight the code
  /// beforehand.
  ///
  /// ## Example
  ///
  /// ```no_run
  /// use hlight::HighlightResource;
  /// use hlight::Highlighter;
  /// use std::fs::File;
  ///
  /// let s = "
  ///   [main]
  ///   enabled = false
  ///   float = 314e-2
  /// ";
  ///
  /// let res = HighlightResource::default().with_background(false);
  /// let mut file = File::create("tmp.txt")?;
  ///
  /// Highlighter::default()
  ///   .with_syntax_name("toml")
  ///   .with_content(s)
  ///   .with_resource(Some(&res))
  ///   .with_writer(Some(&mut file))
  ///   .run()?;
  ///
  /// # Ok::<(), std::io::Error>(())
  /// ```
  pub fn run(self) -> io::Result<()> {
    let Self {
      syntax_name,
      content,
      resource: style,
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
          .get_theme_name()
          .eq_ignore_ascii_case("none") =>
      {
        s
      }
      _ => {
        out.write_all(content.as_bytes())?;
        return out.flush();
      }
    };
    let syntax_set = hl_res.get_syntax_set();

    log::debug!("About to Load the SyntaxSet and ThemeSet");

    let syntax = find_syntax(syntax_set, syntax_name);

    log::trace!("ext: {:?}", syntax.file_extensions);
    log::debug!("syntax: {}", syntax.name);

    let lines = HighlightLines::new(syntax, hl_res.get_or_init_theme());

    write_highlight_line(content, lines, syntax_set, *hl_res.get_background(), out)?;
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
  content: &str,
  mut highlight_lines: HighlightLines,
  syntax_set: &SyntaxSet,
  background: bool,
  writer: &mut dyn Write,
) -> io::Result<()> {
  for line in LinesWithEndings::from(content) {
    let ranges = highlight_lines
      .highlight_line(line, syntax_set)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let escaped = as_24_bit_terminal_escaped(&ranges[..], background);
    writer.write_all(escaped.as_bytes())?
  }
  writer.write_all(b"\x1B[0m")?;
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
    let res = HighlightResource::default();
    Highlighter::default()
      .with_syntax_name("toml")
      .with_resource((&res).into())
      .with_content(S)
      .run()?;
    Ok(())
  }

  #[test]
  #[ignore]
  #[cfg(unix)]
  fn write_to_file() -> io::Result<()> {
    use std::fs::File;

    let res = HighlightResource::default().with_background(false);
    let mut file = File::create("/tmp/test.txt")?;

    Highlighter::default()
      .with_syntax_name("toml")
      .with_content(S)
      .with_resource((&res).into())
      .with_writer(Some(&mut file))
      .run()?;
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

    let res = HighlightResource::default().with_background(false);
    Highlighter::default()
      .with_content(s)
      .with_resource(Some(&res))
      .with_syntax_name("pwsh")
      .run()?;
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

    let res = HighlightResource::default().with_background(true);
    Highlighter::default()
      .with_syntax_name("sh")
      .with_content(s)
      .with_resource((&res).into())
      .run()?;
    Ok(())
  }
}
