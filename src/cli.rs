use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorName {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}

#[derive(Debug, Parser)]
#[command(name = "artgen", version, about = "Generate colored ASCII art text in the terminal")]
pub struct Cli {
    /// Text to render as ASCII art. Accepts one or more words.
    #[arg(required = true, num_args = 1..)]
    pub text: Vec<String>,

    /// Output color.
    #[arg(long, value_enum, default_value_t = ColorName::White)]
    pub color: ColorName,
}
