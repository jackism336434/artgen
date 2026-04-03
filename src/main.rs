

mod cli;
mod color;
mod render;

use clap::Parser;
use std::process::ExitCode;

use cli::Cli;

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
    color::print_colored(&rendered, cli.color).map_err(|err| format!("failed to write to terminal: {err}"))?;
    Ok(())
}
