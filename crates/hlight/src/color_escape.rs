use core::fmt::Write;

use ansi_colours::ansi256_from_rgb;
use syntect::highlighting::{Color, Style};
use tap::Pipe;

/// Blends foreground color with background using alpha compositing
///
/// ## Algorithm
///
/// When foreground alpha is 255 (opaque), returns original foreground.
/// Otherwise blends each RGB channel using formula:
/// (fg_channel * alpha + bg_channel * (255 - alpha)) / 255
fn blend_fg_color(fg: Color, bg: Color) -> Color {
  let ratio = match fg.a {
    0xff => return fg, // Fully opaque, no blending needed
    x => x as u32,     // Use alpha value as blending ratio
  };

  // Channel-wise blending closure
  let blend = |fg_channel, bg_channel| {
    ((fg_channel as u32 * ratio + bg_channel as u32 * (255 - ratio)) / 255) as u8
  };

  let (r, g, b) = (
    blend(fg.r, bg.r), // Red channel blending
    blend(fg.g, bg.g), // Green channel blending
    blend(fg.b, bg.b), // Blue channel blending
  );
  Color { r, g, b, a: 255 } // Output is always fully opaque
}

/// Converts styled text segments to ANSI 256-color escape sequences
///
/// ## Parameters
///
/// * `sty_text_pairs` - Slice of (Style, text) tuples
/// * `background` - Whether to set background color
///
/// ## Process Flow
///
/// 1. Processes background color if enabled (48 escape code)
/// 2. Blends foreground color with background
/// 3. Converts RGB to ANSI 256-color codes
/// 4. Builds ANSI escape sequences for each text segment
pub fn to_ansi_256color(
  sty_text_pairs: &[(Style, &str)],
  background: bool,
) -> Result<String, core::fmt::Error> {
  let mut s = String::new();

  // Background processing closure
  let process_bg = |bg: Color| {
    let (r, g, b) = (bg.r, bg.g, bg.b);
    (r, g, b).pipe(ansi256_from_rgb) // Direct ANSI code conversion
  };

  // Foreground processing closure
  let process_fg = |sty: &Style| {
    blend_fg_color(sty.foreground, sty.background) // Alpha blending
      .pipe(|c| (c.r, c.g, c.b)) // Extract RGB
      .pipe(ansi256_from_rgb) // Convert to ANSI code
  };

  // Iterate through style-text pairs
  sty_text_pairs
    .iter()
    .try_for_each(|(style, text)| {
      // Set background color (48 escape code)
      if background {
        style
          .background
          .pipe(process_bg)
          .pipe(|ansi_bg| write!(s, "\x1b[48;5;{ansi_bg}m"))?
      }

      // Set foreground color (38 escape code)
      process_fg(style).pipe(|ansi_fg| write!(s, "\x1b[38;5;{ansi_fg}m{text}"))
    })?;

  Ok(s)
}
