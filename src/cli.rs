use clap::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorName {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliColor {
    Named(ColorName),
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
}

impl CliColor {
    pub fn white() -> Self {
        Self::Named(ColorName::White)
    }

    pub fn black() -> Self {
        Self::Rgb { r: 0, g: 0, b: 0 }
    }

    pub fn resolve(self, background: (u8, u8, u8)) -> Self {
        match self {
            Self::Named(_) | Self::Rgb { .. } => self,
            Self::Rgba { r, g, b, a } => Self::Rgb {
                r: blend_channel_against_background(r, background.0, a),
                g: blend_channel_against_background(g, background.1, a),
                b: blend_channel_against_background(b, background.2, a),
            },
        }
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            Self::Named(color_name) => color_name_to_rgb(color_name),
            Self::Rgb { r, g, b } => (r, g, b),
            Self::Rgba { .. } => self.resolve((0, 0, 0)).to_rgb(),
        }
    }
}

fn blend_channel_against_background(channel: u8, background: u8, alpha: u8) -> u8 {
    let foreground = channel as u16 * alpha as u16;
    let background = background as u16 * (255 - alpha) as u16;
    ((foreground + background + 127) / 255) as u8
}

impl FromStr for CliColor {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = value.strip_prefix('#') {
            return parse_hex_color(hex);
        }

        if value.len() >= 5
            && value
                .get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("rgb("))
            && value.ends_with(')')
        {
            return parse_rgb_function_color(value);
        }

        if value.contains(',') {
            return parse_rgb_csv_color(value);
        }

        match value.to_ascii_lowercase().as_str() {
            "red" => Ok(Self::Named(ColorName::Red)),
            "green" => Ok(Self::Named(ColorName::Green)),
            "blue" => Ok(Self::Named(ColorName::Blue)),
            "yellow" => Ok(Self::Named(ColorName::Yellow)),
            "cyan" => Ok(Self::Named(ColorName::Cyan)),
            "magenta" => Ok(Self::Named(ColorName::Magenta)),
            "white" => Ok(Self::Named(ColorName::White)),
            _ => Err(
                "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b"
                    .to_string(),
            ),
        }
    }
}

fn parse_hex_color(hex: &str) -> Result<CliColor, String> {
    if hex.len() == 3 {
        return parse_short_hex_color(hex);
    }

    if hex.len() == 4 {
        return parse_short_hex_alpha_color(hex);
    }

    if hex.len() == 8 {
        return parse_hex_alpha_color(hex);
    }

    if hex.len() != 6 {
        return Err(
            "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b"
                .to_string(),
        );
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })?;

    Ok(CliColor::Rgb { r, g, b })
}

fn parse_short_hex_color(hex: &str) -> Result<CliColor, String> {
    let digits: Vec<char> = hex.chars().collect();
    let r = parse_short_hex_channel(digits[0])?;
    let g = parse_short_hex_channel(digits[1])?;
    let b = parse_short_hex_channel(digits[2])?;

    Ok(CliColor::Rgb { r, g, b })
}

fn parse_short_hex_channel(value: char) -> Result<u8, String> {
    let digit = value.to_digit(16).ok_or_else(|| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })? as u8;
    Ok(digit * 17)
}

fn parse_short_hex_alpha_color(hex: &str) -> Result<CliColor, String> {
    let digits: Vec<char> = hex.chars().collect();
    let r = parse_short_hex_channel(digits[0])?;
    let g = parse_short_hex_channel(digits[1])?;
    let b = parse_short_hex_channel(digits[2])?;
    let alpha = parse_short_hex_channel(digits[3])?;

    Ok(CliColor::Rgba { r, g, b, a: alpha })
}

fn parse_hex_alpha_color(hex: &str) -> Result<CliColor, String> {
    let r = parse_hex_byte(&hex[0..2])?;
    let g = parse_hex_byte(&hex[2..4])?;
    let b = parse_hex_byte(&hex[4..6])?;
    let alpha = parse_hex_byte(&hex[6..8])?;

    Ok(CliColor::Rgba { r, g, b, a: alpha })
}

fn parse_hex_byte(value: &str) -> Result<u8, String> {
    u8::from_str_radix(value, 16).map_err(|_| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })
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

fn parse_rgb_function_color(value: &str) -> Result<CliColor, String> {
    parse_rgb_channels(&value[4..value.len() - 1])
}

fn parse_rgb_csv_color(value: &str) -> Result<CliColor, String> {
    parse_rgb_channels(value)
}

fn parse_rgb_channels(value: &str) -> Result<CliColor, String> {
    let channels: Vec<&str> = value.split(',').map(str::trim).collect();
    if channels.len() != 3 {
        return Err(
            "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b"
                .to_string(),
        );
    }

    let r = parse_rgb_channel(channels[0])?;
    let g = parse_rgb_channel(channels[1])?;
    let b = parse_rgb_channel(channels[2])?;

    Ok(CliColor::Rgb { r, g, b })
}

fn parse_rgb_channel(value: &str) -> Result<u8, String> {
    value.parse::<u8>().map_err(|_| {
        "expected named color, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b".to_string()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum GradientName {
    Rainbow,
    Sunset,
    Ocean,
    Fire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum AnimationName {
    Blink,
    Shine,
}

pub const DEFAULT_ANIMATION_SPEED_MS: u64 = 150;

#[derive(Debug, Parser)]
#[command(
    name = "artgen",
    version,
    about = "Generate colored ASCII art text in the terminal"
)]
pub struct Cli {
    /// Text to render as ASCII art. Accepts one or more words.
    #[arg(required = true, num_args = 1..)]
    pub text: Vec<String>,

    /// Output color. Accepts named colors, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b.
    #[arg(long, conflicts_with = "gradient")]
    pub color: Option<CliColor>,

    /// Output gradient.
    #[arg(long, value_enum, conflicts_with = "color")]
    pub gradient: Option<GradientName>,

    /// Background color used to resolve alpha-enabled colors. Defaults to black.
    #[arg(long)]
    pub alpha_bg: Option<CliColor>,

    /// Custom gradient start color. Accepts named colors, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b.
    #[arg(long, requires = "to", conflicts_with_all = ["color", "gradient"])]
    pub from: Option<CliColor>,

    /// Custom gradient end color. Accepts named colors, #RGB, #RGBA, #RRGGBB, #RRGGBBAA, rgb(r,g,b), or r,g,b.
    #[arg(long, requires = "from", conflicts_with_all = ["color", "gradient"])]
    pub to: Option<CliColor>,

    /// Animation effect.
    #[arg(long, value_enum)]
    pub animate: Option<AnimationName>,

    /// Animation frame interval in milliseconds.
    #[arg(long, requires = "animate", value_parser = clap::value_parser!(u64).range(1..))]
    pub speed: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_named_gradient_presets() {
        let cli =
            Cli::try_parse_from(["artgen", "hello", "world", "--gradient", "sunset"]).unwrap();

        assert_eq!(cli.text, vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(cli.gradient, Some(GradientName::Sunset));
        assert_eq!(cli.color, None);
    }

    #[test]
    fn rejects_color_and_gradient_together() {
        let result =
            Cli::try_parse_from(["artgen", "hello", "--color", "red", "--gradient", "rainbow"]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_custom_gradient_endpoints() {
        let cli =
            Cli::try_parse_from(["artgen", "hello", "--from", "red", "--to", "blue"]).unwrap();

        assert_eq!(cli.from, Some(CliColor::Named(ColorName::Red)));
        assert_eq!(cli.to, Some(CliColor::Named(ColorName::Blue)));
        assert_eq!(cli.color, None);
        assert_eq!(cli.gradient, None);
    }

    #[test]
    fn parses_hex_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#ff6600"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_uppercase_hex_color() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#FF6600"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_short_hex_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#f60"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_uppercase_short_hex_color() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#F6A"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 170
            })
        );
    }

    #[test]
    fn parses_hex_custom_gradient_endpoints() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--from", "#ff0000", "--to", "#0000ff"])
            .unwrap();

        assert_eq!(cli.from, Some(CliColor::Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(cli.to, Some(CliColor::Rgb { r: 0, g: 0, b: 255 }));
    }

    #[test]
    fn parses_short_hex_custom_gradient_endpoints() {
        let cli =
            Cli::try_parse_from(["artgen", "hello", "--from", "#f00", "--to", "#0af"]).unwrap();

        assert_eq!(cli.from, Some(CliColor::Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(
            cli.to,
            Some(CliColor::Rgb {
                r: 0,
                g: 170,
                b: 255
            })
        );
    }

    #[test]
    fn parses_short_hex_alpha_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#f608"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgba {
                r: 255,
                g: 102,
                b: 0,
                a: 136
            })
        );
    }

    #[test]
    fn parses_hex_alpha_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "#ff660080"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgba {
                r: 255,
                g: 102,
                b: 0,
                a: 128
            })
        );
    }

    #[test]
    fn parses_alpha_hex_custom_gradient_endpoints() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--from", "#f00f", "--to", "#00aaff80"])
            .unwrap();

        assert_eq!(
            cli.from,
            Some(CliColor::Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            })
        );
        assert_eq!(
            cli.to,
            Some(CliColor::Rgba {
                r: 0,
                g: 170,
                b: 255,
                a: 128
            })
        );
    }

    #[test]
    fn parses_alpha_background_color() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--alpha-bg", "white"]).unwrap();

        assert_eq!(cli.alpha_bg, Some(CliColor::Named(ColorName::White)));
    }

    #[test]
    fn parses_hex_alpha_background_color() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--alpha-bg", "#123456"]).unwrap();

        assert_eq!(
            cli.alpha_bg,
            Some(CliColor::Rgb {
                r: 18,
                g: 52,
                b: 86
            })
        );
    }

    #[test]
    fn alpha_background_can_use_alpha_literals() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--alpha-bg", "#fff8"]).unwrap();

        assert_eq!(
            cli.alpha_bg,
            Some(CliColor::Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 136
            })
        );
    }

    #[test]
    fn resolves_alpha_color_against_black_by_default() {
        assert_eq!(
            CliColor::Rgba {
                r: 255,
                g: 102,
                b: 0,
                a: 128
            }
            .resolve((0, 0, 0)),
            CliColor::Rgb {
                r: 128,
                g: 51,
                b: 0
            }
        );
    }

    #[test]
    fn resolves_alpha_color_against_custom_background() {
        assert_eq!(
            CliColor::Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 128
            }
            .resolve((255, 255, 255)),
            CliColor::Rgb {
                r: 255,
                g: 127,
                b: 127
            }
        );
    }

    #[test]
    fn parses_rgb_function_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "rgb(255,102,0)"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_rgb_csv_color_for_solid_output() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "255, 102, 0"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_case_insensitive_rgb_function() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--color", "RGB(255,102,0)"]).unwrap();

        assert_eq!(
            cli.color,
            Some(CliColor::Rgb {
                r: 255,
                g: 102,
                b: 0
            })
        );
    }

    #[test]
    fn parses_rgb_function_custom_gradient_endpoints() {
        let cli = Cli::try_parse_from([
            "artgen",
            "hello",
            "--from",
            "rgb(255,0,0)",
            "--to",
            "0,0,255",
        ])
        .unwrap();

        assert_eq!(cli.from, Some(CliColor::Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(cli.to, Some(CliColor::Rgb { r: 0, g: 0, b: 255 }));
    }

    #[test]
    fn rejects_missing_to_for_custom_gradient() {
        let result = Cli::try_parse_from(["artgen", "hello", "--from", "red"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_hex_without_hash_prefix() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "ff6600"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_short_hex_color() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "#12"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_short_hex_color() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "#ggg"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_short_hex_alpha_color() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "#gggg"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_hex_alpha_color() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "#ff6600zz"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_hex_color() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "#zzz999"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_rgb_function_with_missing_channel() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "rgb(255,0)"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_rgb_csv_with_extra_channel() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "255,0,0,10"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_rgb_function_with_non_numeric_channel() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "rgb(255,blue,0)"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_rgb_channel_out_of_range() {
        let result = Cli::try_parse_from(["artgen", "hello", "--color", "rgb(256,0,0)"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_custom_gradient_with_preset_gradient() {
        let result = Cli::try_parse_from([
            "artgen",
            "hello",
            "--from",
            "red",
            "--to",
            "blue",
            "--gradient",
            "fire",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_blink_animation_with_gradient() {
        let cli = Cli::try_parse_from([
            "artgen",
            "hello",
            "--gradient",
            "rainbow",
            "--animate",
            "blink",
        ])
        .unwrap();

        assert_eq!(cli.gradient, Some(GradientName::Rainbow));
        assert_eq!(cli.animate, Some(AnimationName::Blink));
    }

    #[test]
    fn parses_animation_speed() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--animate", "blink", "--speed", "120"])
            .unwrap();

        assert_eq!(cli.animate, Some(AnimationName::Blink));
        assert_eq!(cli.speed, Some(120));
    }

    #[test]
    fn rejects_speed_without_animation() {
        let result = Cli::try_parse_from(["artgen", "hello", "--speed", "150"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_speed() {
        let result = Cli::try_parse_from(["artgen", "hello", "--animate", "blink", "--speed", "0"]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_shine_animation_with_speed() {
        let cli = Cli::try_parse_from(["artgen", "hello", "--animate", "shine", "--speed", "90"])
            .unwrap();

        assert_eq!(cli.animate, Some(AnimationName::Shine));
        assert_eq!(cli.speed, Some(90));
    }
}
