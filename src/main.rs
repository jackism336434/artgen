mod cli;
mod color;
mod render;

use clap::Parser;
use std::process::ExitCode;

use cli::{Cli, ColorName};
use color::OutputStyle;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let text = cli.text.join(" ");
    let rendered = render::render_text(&text)?;
    let style = match (cli.gradient, cli.from, cli.to) {
        (Some(gradient), None, None) => OutputStyle::Gradient(gradient),
        (None, Some(from), Some(to)) => OutputStyle::CustomGradient { from, to },
        (None, None, None) => OutputStyle::Solid(cli.color.unwrap_or(ColorName::White)),
        _ => return Err("invalid color mode combination".to_string()),
    };

    color::print_styled(&rendered, style)
        .map_err(|err| format!("failed to write to terminal: {err}"))?;
    Ok(())
}
