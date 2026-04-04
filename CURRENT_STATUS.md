# Current Status

## Current Phase
The project is in the early CLI foundation stage.

The base workflow already works:

- parse CLI input
- render ASCII art with `figlet-rs`
- print solid colors
- print left-to-right preset gradients

## Completed Work
- Initial CLI tool structure created with `main.rs`, `cli.rs`, `render.rs`, and `color.rs`
- Added `figlet-rs` rendering
- Added solid color output with `crossterm`
- Fixed multi-word text input so quotes are optional
- Added `--gradient rainbow`
- Expanded `--gradient` presets with `sunset`, `ocean`, and `fire`
- Added custom two-endpoint gradients with `--from <color> --to <color>`
- Enforced mutual exclusion between `--color` and `--gradient`
- Created `plans/` directory for incremental task planning
- Updated user-facing README to match current CLI behavior
- Added unit tests for CLI parsing and gradient palette mapping

## Recent Plan Files
- `plans/2026-04-04-preset-gradients.md`
- `plans/2026-04-04-two-endpoint-gradient.md`
- `plans/2026-04-04-cli-text-input.md`
- `plans/2026-04-04-rainbow-gradient.md`
- `plans/2026-04-04-project-workflow.md`

## Current CLI Behavior
- Accepts one or more words as text input
- Supports `--color <name>` for solid output
- Supports `--gradient <name>` for left-to-right preset gradients
- Supports `--from <color> --to <color>` for left-to-right custom gradients
- Defaults to white solid output when no color mode is provided

Supported gradients:

- `rainbow`
- `sunset`
- `ocean`
- `fire`

Custom endpoint colors currently reuse the existing named colors:

- `red`
- `green`
- `blue`
- `yellow`
- `cyan`
- `magenta`
- `white`

## Next Recommended Steps
- Consider whether the next color step should add hex/RGB input or more named colors for custom endpoints
- Consider lightweight integration tests around `cargo run` behavior in addition to current unit tests
- Revisit animation only after color behavior and terminal compatibility expectations are clearer

## Known Workflow Rule
Before adding any new feature, create a matching plan file under `plans/`.

At the start of a new session, recover context by reading:

1. `PROJECT_RULES.md`
2. `CURRENT_STATUS.md`
3. the latest relevant files in `plans/`
