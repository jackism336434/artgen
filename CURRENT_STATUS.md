# Current Status

## Current Phase
The project is in the early CLI foundation stage.

The base workflow already works:

- parse CLI input
- render ASCII art with `figlet-rs`
- print solid colors
- print left-to-right preset gradients
- print left-to-right custom two-endpoint gradients
- animate blink effects over existing color modes
- animate a narrow left-to-right shine highlight over existing color modes without changing the base color outside the highlight band
- configure animation frame speed with a shared CLI parameter
- reduce terminal flicker by overwriting frames instead of clearing the whole animation area each tick

## Completed Work
- Initial CLI tool structure created with `main.rs`, `cli.rs`, `render.rs`, and `color.rs`
- Added `figlet-rs` rendering
- Added solid color output with `crossterm`
- Fixed multi-word text input so quotes are optional
- Added `--gradient rainbow`
- Expanded `--gradient` presets with `sunset`, `ocean`, and `fire`
- Added custom two-endpoint gradients with `--from <color> --to <color>`
- Added `--animate blink` with frame-based terminal redraw
- Added `--animate shine` with a narrow white sweep effect
- Refined `shine` so only a very narrow highlight band changes and the base color stays fixed outside it
- Added shared `--speed <ms>` animation timing control
- Changed animation redraw strategy to overwrite frames instead of clearing the full region on every tick
- Enforced mutual exclusion between `--color` and `--gradient`
- Created `plans/` directory for incremental task planning
- Updated user-facing README to match current CLI behavior
- Added unit tests for CLI parsing, gradient palette mapping, and animation helpers

## Recent Plan Files
- `plans/2026-04-04-shine-animation.md`
- `plans/2026-04-04-shine-refine.md`
- `plans/2026-04-04-animation-redraw.md`
- `plans/2026-04-04-animation-speed.md`
- `plans/2026-04-04-blink-animation.md`
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
- Supports `--animate blink|shine` to animate the selected color mode until `Ctrl+C`
- Supports `--speed <ms>` to control animation frame interval
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
- Consider whether the next animation should be `scroll` or a randomized `twinkle`
- Consider lightweight integration tests around `cargo run` behavior in addition to current unit tests
- Consider whether color input should expand to hex/RGB before adding more animation styles

## Known Workflow Rule
Before adding any new feature, create a matching plan file under `plans/`.

At the start of a new session, recover context by reading:

1. `PROJECT_RULES.md`
2. `CURRENT_STATUS.md`
3. the latest relevant files in `plans/`
