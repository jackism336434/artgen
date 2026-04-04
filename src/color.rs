use std::io::{self, Write};

use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};

use crate::cli::{ColorName, GradientName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStyle {
    Solid(ColorName),
    Gradient(GradientName),
    CustomGradient { from: ColorName, to: ColorName },
}

pub fn print_styled(text: &str, style: OutputStyle) -> io::Result<()> {
    match style {
        OutputStyle::Solid(color_name) => print_solid(text, color_name),
        OutputStyle::Gradient(gradient_name) => print_gradient(text, gradient_name),
        OutputStyle::CustomGradient { from, to } => print_two_color_gradient(text, from, to),
    }
}

fn print_solid(text: &str, color_name: ColorName) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(SetForegroundColor(to_crossterm_color(color_name)))?;
    write!(stdout, "{text}")?;
    stdout.execute(ResetColor)?;
    writeln!(stdout)?;
    Ok(())
}

fn print_gradient(text: &str, gradient_name: GradientName) -> io::Result<()> {
    print_gradient_with_picker(text, |column, width| {
        let palette = palette_for_gradient(gradient_name);
        gradient_color_for_column(column, width, palette)
    })
}

fn print_two_color_gradient(text: &str, from: ColorName, to: ColorName) -> io::Result<()> {
    let start = color_name_to_rgb(from);
    let end = color_name_to_rgb(to);

    print_gradient_with_picker(text, |column, width| {
        interpolated_color_for_column(column, width, start, end)
    })
}

fn print_gradient_with_picker<F>(text: &str, mut color_for_column: F) -> io::Result<()>
where
    F: FnMut(usize, usize) -> Color,
{
    let mut stdout = io::stdout();
    let lines: Vec<&str> = text.lines().collect();
    let max_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    for line in lines {
        for (column, ch) in line.chars().enumerate() {
            stdout.execute(SetForegroundColor(color_for_column(column, max_width)))?;
            write!(stdout, "{ch}")?;
        }
        stdout.execute(ResetColor)?;
        writeln!(stdout)?;
    }

    stdout.execute(ResetColor)?;
    Ok(())
}

fn palette_for_gradient(gradient_name: GradientName) -> &'static [Color] {
    const RAINBOW: [Color; 6] = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];
    const SUNSET: [Color; 5] = [
        Color::Rgb {
            r: 255,
            g: 94,
            b: 87,
        },
        Color::Rgb {
            r: 255,
            g: 149,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 204,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 99,
            b: 132,
        },
        Color::Rgb {
            r: 143,
            g: 76,
            b: 255,
        },
    ];
    const OCEAN: [Color; 5] = [
        Color::Rgb {
            r: 0,
            g: 95,
            b: 115,
        },
        Color::Rgb {
            r: 10,
            g: 147,
            b: 150,
        },
        Color::Rgb {
            r: 0,
            g: 180,
            b: 216,
        },
        Color::Rgb {
            r: 72,
            g: 202,
            b: 228,
        },
        Color::Rgb {
            r: 144,
            g: 224,
            b: 239,
        },
    ];
    const FIRE: [Color; 5] = [
        Color::Rgb {
            r: 255,
            g: 59,
            b: 48,
        },
        Color::Rgb {
            r: 255,
            g: 110,
            b: 64,
        },
        Color::Rgb {
            r: 255,
            g: 159,
            b: 10,
        },
        Color::Rgb {
            r: 255,
            g: 214,
            b: 10,
        },
        Color::Rgb {
            r: 255,
            g: 240,
            b: 181,
        },
    ];

    match gradient_name {
        GradientName::Rainbow => &RAINBOW,
        GradientName::Sunset => &SUNSET,
        GradientName::Ocean => &OCEAN,
        GradientName::Fire => &FIRE,
    }
}

fn gradient_color_for_column(column: usize, width: usize, palette: &[Color]) -> Color {
    if width <= 1 || palette.len() <= 1 {
        return palette[0];
    }

    let last_index = palette.len() - 1;
    let palette_index = column * last_index / (width - 1);
    palette[palette_index]
}

fn interpolated_color_for_column(
    column: usize,
    width: usize,
    start: (u8, u8, u8),
    end: (u8, u8, u8),
) -> Color {
    if width <= 1 {
        return Color::Rgb {
            r: start.0,
            g: start.1,
            b: start.2,
        };
    }

    let ratio = column as f32 / (width - 1) as f32;
    Color::Rgb {
        r: interpolate_channel(start.0, end.0, ratio),
        g: interpolate_channel(start.1, end.1, ratio),
        b: interpolate_channel(start.2, end.2, ratio),
    }
}

fn interpolate_channel(start: u8, end: u8, ratio: f32) -> u8 {
    let start = start as f32;
    let end = end as f32;
    (start + (end - start) * ratio).round() as u8
}

fn color_name_to_rgb(color_name: ColorName) -> (u8, u8, u8) {
    match color_name {
        ColorName::Red => (255, 0, 0),
        ColorName::Green => (0, 255, 0),
        ColorName::Blue => (0, 0, 255),
        ColorName::Yellow => (255, 255, 0),
        ColorName::Cyan => (0, 255, 255),
        ColorName::Magenta => (255, 0, 255),
        ColorName::White => (255, 255, 255),
    }
}

fn to_crossterm_color(color_name: ColorName) -> Color {
    match color_name {
        ColorName::Red => Color::Red,
        ColorName::Green => Color::Green,
        ColorName::Blue => Color::Blue,
        ColorName::Yellow => Color::Yellow,
        ColorName::Cyan => Color::Cyan,
        ColorName::Magenta => Color::Magenta,
        ColorName::White => Color::White,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_first_palette_color_for_single_column_output() {
        let palette = palette_for_gradient(GradientName::Fire);

        assert_eq!(gradient_color_for_column(0, 1, palette), palette[0]);
    }

    #[test]
    fn maps_full_width_across_entire_palette() {
        let palette = palette_for_gradient(GradientName::Ocean);

        assert_eq!(gradient_color_for_column(0, 5, palette), palette[0]);
        assert_eq!(
            gradient_color_for_column(4, 5, palette),
            palette[palette.len() - 1]
        );
    }

    #[test]
    fn rainbow_palette_still_uses_expected_terminal_colors() {
        let palette = palette_for_gradient(GradientName::Rainbow);

        assert_eq!(palette[0], Color::Red);
        assert_eq!(palette[palette.len() - 1], Color::Magenta);
    }

    #[test]
    fn single_column_custom_gradient_uses_start_color() {
        assert_eq!(
            interpolated_color_for_column(0, 1, (255, 0, 0), (0, 0, 255)),
            Color::Rgb { r: 255, g: 0, b: 0 }
        );
    }

    #[test]
    fn custom_gradient_uses_start_and_end_colors_at_edges() {
        assert_eq!(
            interpolated_color_for_column(0, 5, (255, 0, 0), (0, 0, 255)),
            Color::Rgb { r: 255, g: 0, b: 0 }
        );
        assert_eq!(
            interpolated_color_for_column(4, 5, (255, 0, 0), (0, 0, 255)),
            Color::Rgb { r: 0, g: 0, b: 255 }
        );
    }

    #[test]
    fn maps_named_colors_to_rgb_triplets() {
        assert_eq!(color_name_to_rgb(ColorName::Cyan), (0, 255, 255));
        assert_eq!(color_name_to_rgb(ColorName::White), (255, 255, 255));
    }
}
