# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build

```bash
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --out-dir pkg --release
```

### Run locally

```bash
python -m http.server 8080
# Open http://localhost:8080
```

### Tests

```bash
# Unit tests (Rust, headless — no browser needed)
cargo test

# Run a single test
cargo test <test_name>

# E2E tests (Playwright + Chromium)
npm install
npx playwright install chromium
npx playwright test
```

### Lint

```bash
cargo clippy
```

## Architecture

The game is a Rust → WASM platformer rendered on an HTML5 Canvas. All graphics are procedural (no sprites).

### Module responsibilities

| Module | Role |
|--------|------|
| `lib.rs` | WASM entry point. Registers keyboard events, drives the game loop via `requestAnimationFrame`. |
| `game.rs` | Game state machine (`GamePhase`: Title / Playing / Dead / Won). Orchestrates `step_logic()`: physics tick, collision detection, coin collection, win/death checks. |
| `player.rs` | Player physics: AABB collision vs. tiles, gravity, jump with cooldown, friction. Key fns: `resolve_x()`, `resolve_y()`. |
| `world.rs` | Level data — 15×210 tile grid, enemy spawning, coins, pipes, bump-bricks. |
| `renderer.rs` | All Canvas 2D drawing. Called once per frame after logic. |
| `input.rs` | Shared `InputState` struct tracking held keys and rise-edge detection. |
| `sim.rs` | Headless `GameSim` that reuses `step_logic` without any Canvas/DOM dependency — used by unit tests. |

### Key design decisions

- **Decoupled logic:** `step_logic()` in `game.rs` is pure game logic, callable from both the browser loop and the headless `GameSim`. This is what makes unit testing possible without WASM.
- **No sprites:** Everything is drawn with `fillRect` / `arc` / strokes on Canvas 2D.
- **Tile grid:** 32×32 px tiles, level is 210 columns × 15 rows. Camera follows the player with clamped bounds.
- **Physics constants:** gravity 0.55 px/frame, max fall 14 px/frame, jump velocity −11.5, jump cooldown 8 frames, friction 0.75.

### CI/CD

- **`test.yml`** — runs on every push/PR to `master`: `cargo test` + `wasm-pack build` + Playwright E2E.
- **`deploy.yml`** — runs on every push to `main`: builds WASM and deploys to GitHub Pages automatically.
