use std::io::{self, Write};

use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};

use crate::cli::ColorName;

pub fn print_colored(text: &str, color_name: ColorName) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(SetForegroundColor(to_crossterm_color(color_name)))?;
    write!(stdout, "{text}")?;
    stdout.execute(ResetColor)?;
    writeln!(stdout)?;
    Ok(())
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
