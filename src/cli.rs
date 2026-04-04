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
}
