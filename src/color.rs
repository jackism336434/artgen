use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    style::{Color, ResetColor, SetForegroundColor},
};

use crate::cli::{AnimationName, CliColor, ColorName, GradientName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStyle {
    Solid(CliColor),
    Gradient(GradientName),
    CustomGradient { from: CliColor, to: CliColor },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationConfig {
    pub name: AnimationName,
    pub frame_interval: Duration,
}

pub fn print_styled(
    text: &str,
    style: OutputStyle,
    animation: Option<AnimationConfig>,
) -> io::Result<()> {
    match animation {
        Some(animation) => print_animated(text, style, animation),
        None => print_static(text, style),
    }
}

fn print_static(text: &str, style: OutputStyle) -> io::Result<()> {
    let mut stdout = io::stdout();
    render_frame_with_transform(&mut stdout, text, style, |_, _, base| base)
}

fn print_animated(text: &str, style: OutputStyle, animation: AnimationConfig) -> io::Result<()> {
    match animation.name {
        AnimationName::Blink => print_blink_animation(text, style, animation.frame_interval),
        AnimationName::Shine => print_shine_animation(text, style, animation.frame_interval),
    }
}

fn print_blink_animation(
    text: &str,
    style: OutputStyle,
    frame_interval: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let line_count = line_count(text);
    let interrupted = install_ctrlc_handler()?;
    let result = (|| -> io::Result<()> {
        stdout.execute(Hide)?;

        while !interrupted.load(Ordering::SeqCst) {
            render_frame_with_transform(&mut stdout, text, style, |_, _, base| base)?;
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            std::thread::sleep(frame_interval);
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            rewind_animation_frame(&mut stdout, line_count)?;
            render_frame_with_transform(&mut stdout, text, style, |_, _, base| {
                adjust_brightness(base, 0.35)
            })?;
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            std::thread::sleep(frame_interval);
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            rewind_animation_frame(&mut stdout, line_count)?;
        }

        Ok(())
    })();

    stdout.execute(ResetColor)?;
    stdout.execute(Show)?;
    writeln!(stdout)?;
    stdout.flush()?;
    result
}

fn print_shine_animation(
    text: &str,
    style: OutputStyle,
    frame_interval: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let line_count = line_count(text);
    let width = text_width(text) as i32;
    let radius = shine_radius(width);
    let interrupted = install_ctrlc_handler()?;
    let result = (|| -> io::Result<()> {
        let mut center = -radius;
        let max_center = width - 1 + radius;
        stdout.execute(Hide)?;

        while !interrupted.load(Ordering::SeqCst) {
            render_frame_with_transform(&mut stdout, text, style, |column, _, base| {
                apply_shine_highlight(base, column, center as f32, radius as f32)
            })?;
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            std::thread::sleep(frame_interval);
            if interrupted.load(Ordering::SeqCst) {
                break;
            }

            rewind_animation_frame(&mut stdout, line_count)?;
            center += 1;
            if center > max_center {
                center = -radius;
            }
        }

        Ok(())
    })();

    stdout.execute(ResetColor)?;
    stdout.execute(Show)?;
    writeln!(stdout)?;
    stdout.flush()?;
    result
}

fn render_frame_with_transform<W: Write, F>(
    stdout: &mut W,
    text: &str,
    style: OutputStyle,
    mut transform: F,
) -> io::Result<()>
where
    F: FnMut(usize, usize, Color) -> Color,
{
    let lines: Vec<&str> = text.lines().collect();
    let max_width = text_width(text);

    for line in lines {
        for (column, ch) in line.chars().enumerate() {
            let base_color = color_for_position(style, column, max_width);
            let color = transform(column, max_width, base_color);
            crossterm::queue!(stdout, SetForegroundColor(color))?;
            write!(stdout, "{ch}")?;
        }
        for _ in line.chars().count()..max_width {
            crossterm::queue!(stdout, ResetColor)?;
            write!(stdout, " ")?;
        }
        crossterm::queue!(stdout, ResetColor)?;
        writeln!(stdout)?;
    }

    crossterm::queue!(stdout, ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn line_count(text: &str) -> u16 {
    text.lines().count() as u16
}

fn text_width(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

fn shine_radius(width: i32) -> i32 {
    if width <= 24 { 2 } else { 3 }
}

fn rewind_animation_frame<W: Write>(stdout: &mut W, line_count: u16) -> io::Result<()> {
    crossterm::queue!(stdout, MoveUp(line_count), MoveToColumn(0))?;
    stdout.flush()?;
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

fn color_for_position(style: OutputStyle, column: usize, width: usize) -> Color {
    match style {
        OutputStyle::Solid(color) => to_crossterm_color(color),
        OutputStyle::Gradient(gradient_name) => {
            let palette = palette_for_gradient(gradient_name);
            gradient_color_for_column(column, width, palette)
        }
        OutputStyle::CustomGradient { from, to } => {
            let start = cli_color_to_rgb(from);
            let end = cli_color_to_rgb(to);
            interpolated_color_for_column(column, width, start, end)
        }
    }
}

fn adjust_brightness(color: Color, brightness: f32) -> Color {
    let (r, g, b) = color_to_rgb(color);
    Color::Rgb {
        r: scale_channel(r, brightness),
        g: scale_channel(g, brightness),
        b: scale_channel(b, brightness),
    }
}

fn apply_shine_highlight(color: Color, column: usize, center: f32, radius: f32) -> Color {
    let distance = (column as f32 - center).abs();
    if distance > radius {
        return color;
    }

    let strength = if distance <= 0.5 {
        0.95
    } else if distance <= 1.5 {
        0.5
    } else {
        0.22
    };

    blend_towards_white(color, strength)
}

fn blend_towards_white(color: Color, amount: f32) -> Color {
    let (r, g, b) = color_to_rgb(color);
    Color::Rgb {
        r: blend_channel_towards_white(r, amount),
        g: blend_channel_towards_white(g, amount),
        b: blend_channel_towards_white(b, amount),
    }
}

fn blend_channel_towards_white(channel: u8, amount: f32) -> u8 {
    let channel = channel as f32;
    (channel + (255.0 - channel) * amount).round() as u8
}

fn scale_channel(channel: u8, brightness: f32) -> u8 {
    ((channel as f32 * brightness).round()).clamp(0.0, 255.0) as u8
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

fn cli_color_to_rgb(color: CliColor) -> (u8, u8, u8) {
    color.to_rgb()
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Black => (0, 0, 0),
        Color::DarkGrey => (85, 85, 85),
        Color::Grey => (128, 128, 128),
        Color::White => (255, 255, 255),
        Color::DarkRed => (128, 0, 0),
        Color::Red => (255, 0, 0),
        Color::DarkGreen => (0, 128, 0),
        Color::Green => (0, 255, 0),
        Color::DarkYellow => (128, 128, 0),
        Color::Yellow => (255, 255, 0),
        Color::DarkBlue => (0, 0, 128),
        Color::Blue => (0, 0, 255),
        Color::DarkMagenta => (128, 0, 128),
        Color::Magenta => (255, 0, 255),
        Color::DarkCyan => (0, 128, 128),
        Color::Cyan => (0, 255, 255),
        Color::Rgb { r, g, b } => (r, g, b),
        Color::AnsiValue(value) => ansi_value_to_rgb(value),
        Color::Reset => (255, 255, 255),
    }
}

fn ansi_value_to_rgb(value: u8) -> (u8, u8, u8) {
    if value < 16 {
        const ANSI_BASE: [(u8, u8, u8); 16] = [
            (0, 0, 0),
            (128, 0, 0),
            (0, 128, 0),
            (128, 128, 0),
            (0, 0, 128),
            (128, 0, 128),
            (0, 128, 128),
            (192, 192, 192),
            (128, 128, 128),
            (255, 0, 0),
            (0, 255, 0),
            (255, 255, 0),
            (0, 0, 255),
            (255, 0, 255),
            (0, 255, 255),
            (255, 255, 255),
        ];
        return ANSI_BASE[value as usize];
    }

    if value >= 232 {
        let shade = 8 + (value - 232) * 10;
        return (shade, shade, shade);
    }

    let value = value - 16;
    let r = value / 36;
    let g = (value % 36) / 6;
    let b = value % 6;
    (
        ansi_component_to_rgb(r),
        ansi_component_to_rgb(g),
        ansi_component_to_rgb(b),
    )
}

fn ansi_component_to_rgb(component: u8) -> u8 {
    match component {
        0 => 0,
        _ => 55 + component * 40,
    }
}

fn to_crossterm_color(color: CliColor) -> Color {
    match color {
        CliColor::Named(ColorName::Red) => Color::Red,
        CliColor::Named(ColorName::Green) => Color::Green,
        CliColor::Named(ColorName::Blue) => Color::Blue,
        CliColor::Named(ColorName::Yellow) => Color::Yellow,
        CliColor::Named(ColorName::Cyan) => Color::Cyan,
        CliColor::Named(ColorName::Magenta) => Color::Magenta,
        CliColor::Named(ColorName::White) => Color::White,
        CliColor::Rgb { r, g, b } => Color::Rgb { r, g, b },
        CliColor::Rgba { .. } => {
            let (r, g, b) = color.to_rgb();
            Color::Rgb { r, g, b }
        }
    }
}

fn install_ctrlc_handler() -> io::Result<Arc<AtomicBool>> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&interrupted);

    ctrlc::set_handler(move || {
        signal.store(true, Ordering::SeqCst);
    })
    .map_err(|err| io::Error::other(format!("failed to install Ctrl+C handler: {err}")))?;

    Ok(interrupted)
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
        assert_eq!(
            cli_color_to_rgb(CliColor::Named(ColorName::Cyan)),
            (0, 255, 255)
        );
        assert_eq!(
            cli_color_to_rgb(CliColor::Named(ColorName::White)),
            (255, 255, 255)
        );
    }

    #[test]
    fn maps_hex_colors_to_rgb_triplets() {
        assert_eq!(
            cli_color_to_rgb(CliColor::Rgb {
                r: 18,
                g: 171,
                b: 239
            }),
            (18, 171, 239)
        );
    }

    #[test]
    fn alpha_blended_hex_colors_store_darkened_rgb_values() {
        assert_eq!(
            cli_color_to_rgb(CliColor::Rgba {
                r: 255,
                g: 102,
                b: 0,
                a: 128
            }),
            (128, 51, 0)
        );
    }

    #[test]
    fn dims_colors_for_blink_dark_frame() {
        assert_eq!(
            adjust_brightness(
                Color::Rgb {
                    r: 200,
                    g: 100,
                    b: 50
                },
                0.5
            ),
            Color::Rgb {
                r: 100,
                g: 50,
                b: 25
            }
        );
    }

    #[test]
    fn resolves_base_color_for_solid_output() {
        assert_eq!(
            color_for_position(
                OutputStyle::Solid(CliColor::Named(ColorName::Yellow)),
                0,
                10
            ),
            Color::Yellow
        );
    }

    #[test]
    fn resolves_base_color_for_hex_solid_output() {
        assert_eq!(
            color_for_position(
                OutputStyle::Solid(CliColor::Rgb {
                    r: 12,
                    g: 34,
                    b: 56
                }),
                0,
                10
            ),
            Color::Rgb {
                r: 12,
                g: 34,
                b: 56
            }
        );
    }

    #[test]
    fn custom_gradient_base_color_keeps_edge_colors() {
        assert_eq!(
            color_for_position(
                OutputStyle::CustomGradient {
                    from: CliColor::Named(ColorName::Red),
                    to: CliColor::Named(ColorName::Blue),
                },
                0,
                5
            ),
            Color::Rgb { r: 255, g: 0, b: 0 }
        );
        assert_eq!(
            color_for_position(
                OutputStyle::CustomGradient {
                    from: CliColor::Named(ColorName::Red),
                    to: CliColor::Named(ColorName::Blue),
                },
                4,
                5
            ),
            Color::Rgb { r: 0, g: 0, b: 255 }
        );
    }

    #[test]
    fn custom_gradient_base_color_keeps_hex_edge_colors() {
        assert_eq!(
            color_for_position(
                OutputStyle::CustomGradient {
                    from: CliColor::Rgb {
                        r: 255,
                        g: 102,
                        b: 0
                    },
                    to: CliColor::Rgb {
                        r: 0,
                        g: 170,
                        b: 255
                    },
                },
                0,
                5
            ),
            Color::Rgb {
                r: 255,
                g: 102,
                b: 0
            }
        );
        assert_eq!(
            color_for_position(
                OutputStyle::CustomGradient {
                    from: CliColor::Rgb {
                        r: 255,
                        g: 102,
                        b: 0
                    },
                    to: CliColor::Rgb {
                        r: 0,
                        g: 170,
                        b: 255
                    },
                },
                4,
                5
            ),
            Color::Rgb {
                r: 0,
                g: 170,
                b: 255
            }
        );
    }

    #[test]
    fn animation_config_stores_frame_interval() {
        let config = AnimationConfig {
            name: AnimationName::Blink,
            frame_interval: Duration::from_millis(175),
        };

        assert_eq!(config.name, AnimationName::Blink);
        assert_eq!(config.frame_interval, Duration::from_millis(175));
    }

    #[test]
    fn shine_highlight_center_is_stronger_than_edge() {
        let center = apply_shine_highlight(
            Color::Rgb {
                r: 80,
                g: 120,
                b: 160,
            },
            5,
            5.0,
            3.0,
        );
        let edge = apply_shine_highlight(
            Color::Rgb {
                r: 80,
                g: 120,
                b: 160,
            },
            7,
            5.0,
            3.0,
        );

        assert_eq!(
            center,
            Color::Rgb {
                r: 246,
                g: 248,
                b: 250
            }
        );
        assert_eq!(
            edge,
            Color::Rgb {
                r: 119,
                g: 150,
                b: 181
            }
        );
    }

    #[test]
    fn shine_highlight_leaves_outside_band_unchanged() {
        let base = Color::Rgb {
            r: 80,
            g: 120,
            b: 160,
        };

        assert_eq!(apply_shine_highlight(base, 12, 5.0, 3.0), base);
    }

    #[test]
    fn shine_radius_stays_narrow() {
        assert_eq!(shine_radius(5), 2);
        assert_eq!(shine_radius(24), 2);
        assert_eq!(shine_radius(25), 3);
    }
}
