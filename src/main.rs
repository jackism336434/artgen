mod cli;
mod color;
mod render;

use clap::Parser;
use std::process::ExitCode;

use cli::{Cli, CliColor, DEFAULT_ANIMATION_SPEED_MS};
use color::{AnimationConfig, OutputStyle};

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
    let alpha_background = cli
        .alpha_bg
        .unwrap_or(CliColor::black())
        .resolve((0, 0, 0))
        .to_rgb();
    let style = match (cli.gradient, cli.from, cli.to) {
        (Some(gradient), None, None) => OutputStyle::Gradient(gradient),
        (None, Some(from), Some(to)) => OutputStyle::CustomGradient {
            from: from.resolve(alpha_background),
            to: to.resolve(alpha_background),
        },
        (None, None, None) => OutputStyle::Solid(
            cli.color
                .unwrap_or(CliColor::white())
                .resolve(alpha_background),
        ),
        _ => return Err("invalid color mode combination".to_string()),
    };
    let animation = cli.animate.map(|name| AnimationConfig {
        name,
        frame_interval: std::time::Duration::from_millis(
            cli.speed.unwrap_or(DEFAULT_ANIMATION_SPEED_MS),
        ),
    });

    color::print_styled(&rendered, style, animation)
        .map_err(|err| format!("failed to write to terminal: {err}"))?;
    Ok(())
}
