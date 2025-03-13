use std::io::{self, BufWriter, Write};

use getset::{Getters, WithSetters};
use syntect::{
  easy::HighlightLines, parsing::SyntaxSet, util::as_24_bit_terminal_escaped,
};
use tap::Pipe;

use crate::{
  color_escape::to_ansi_256color, resource::HighlightResource, syntax::find_syntax,
};

#[derive(Getters, WithSetters)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct Highlighter<'a, 'w> {
  /// target syntax format (e.g., "json")
  syntax_name: &'a str,
  content: &'a str,
  resource: Option<&'a HighlightResource<'a>>,
  writer: Option<&'w mut dyn Write>,
  true_color: bool,
}

impl Default for Highlighter<'_, '_> {
  fn default() -> Self {
    Self {
      syntax_name: "markdown",
      content: "",
      resource: None, //Some(HighlightResource::default()),
      writer: None,
      true_color: true,
    }
  }
}

impl Highlighter<'_, '_> {
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
      resource,
      writer,
      true_color,
    } = self;

    let mut stdout = std::io::stdout().pipe(BufWriter::new);

    let out = match writer {
      Some(w) => {
        drop(stdout);
        w
      }
      _ => &mut stdout as &mut dyn Write,
    };

    let hl_res = match resource {
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

    // let lines = HighlightLines::new(syntax, hl_res.get_or_init_theme());

    HighlightConfig::default()
      .with_background(*hl_res.get_background())
      .with_content(content)
      .with_syntax_set(Some(syntax_set))
      .with_true_color(true_color)
      .with_highlight_lines(
        HighlightLines::new(syntax, hl_res.get_or_init_theme()).into(),
      )
      .write_lines(out)?;

    out.flush()?;

    log::debug!("Output complete");
    Ok(())
  }
}

#[derive(Getters, WithSetters, Default)]
#[getset(get = "with_prefix", set_with)]
struct HighlightConfig<'a> {
  content: &'a str,
  highlight_lines: Option<HighlightLines<'a>>,
  syntax_set: Option<&'a SyntaxSet>,
  background: bool,
  true_color: bool,
}

impl HighlightConfig<'_> {
  /// Performs the actual highlighting of lines of code, and writes the
  /// highlighted output to the specified output stream.
  fn write_lines(self, writer: &mut dyn Write) -> io::Result<()> {
    let Self {
      content,
      highlight_lines,
      syntax_set,
      background,
      true_color,
    } = self;

    let invalid_data_err = |e| io::Error::new(io::ErrorKind::InvalidData, e);

    let escaped_err = |e| {
      format!("Failed to escape text with ANSI-256-color.\n Error: {e}")
        .pipe(invalid_data_err)
    };

    let opt_err = || {
      "Failed to unwrap Some(data)"
        .to_owned()
        .pipe(invalid_data_err)
    };

    let mut highlight_lines = highlight_lines.ok_or_else(opt_err)?;
    let syntax_set = syntax_set.ok_or_else(opt_err)?;

    content
      .split_inclusive('\n')
      .try_for_each(|line| {
        let ranges = highlight_lines
          .highlight_line(line, syntax_set)
          .map_err(|e| {
            e.to_string()
              .pipe(invalid_data_err)
          })?;

        let write_all = |data| writer.write_all(data);

        match true_color {
          true => as_24_bit_terminal_escaped(&ranges, background),
          _ => to_ansi_256color(&ranges, background).map_err(escaped_err)?,
        }
        .as_bytes()
        .pipe(write_all)
      })?;
    writer.write_all(b"\x1B[0m")
  }
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
    let res = HighlightResource::default() //
      .with_background(false);

    Highlighter::default()
      .with_syntax_name("toml")
      .with_resource((&res).into())
      .with_content(S)
      .with_true_color(false)
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
