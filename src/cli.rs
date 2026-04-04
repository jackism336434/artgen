use clap::{Parser, ValueEnum};

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

    /// Output color.
    #[arg(long, value_enum, conflicts_with = "gradient")]
    pub color: Option<ColorName>,

    /// Output gradient.
    #[arg(long, value_enum, conflicts_with = "color")]
    pub gradient: Option<GradientName>,

    /// Custom gradient start color.
    #[arg(long, value_enum, requires = "to", conflicts_with_all = ["color", "gradient"])]
    pub from: Option<ColorName>,

    /// Custom gradient end color.
    #[arg(long, value_enum, requires = "from", conflicts_with_all = ["color", "gradient"])]
    pub to: Option<ColorName>,

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

        assert_eq!(cli.from, Some(ColorName::Red));
        assert_eq!(cli.to, Some(ColorName::Blue));
        assert_eq!(cli.color, None);
        assert_eq!(cli.gradient, None);
    }

    #[test]
    fn rejects_missing_to_for_custom_gradient() {
        let result = Cli::try_parse_from(["artgen", "hello", "--from", "red"]);

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
