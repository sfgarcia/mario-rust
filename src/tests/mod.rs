/// Tests de lógica de juego — se ejecutan en el target nativo (no WASM).
/// No dependen de web-sys ni del DOM.

mod enemies;
mod game;
mod html;
mod player;
mod world;

// ── Helpers compartidos ──────────────────────────────────────────────────────

use crate::input::InputState;
use crate::player::Player;
use crate::world::{Coin, Enemy, World};

pub(super) fn world() -> World {
    World::new()
}

pub(super) fn no_input() -> InputState {
    InputState::default()
}

pub(super) fn right_input() -> InputState {
    InputState { right: true, ..Default::default() }
}

pub(super) fn left_input() -> InputState {
    InputState { left: true, ..Default::default() }
}

pub(super) fn jump_input() -> InputState {
    InputState { jump: true, jump_pressed: true, ..Default::default() }
}

pub(super) fn player_at(x: f64, y: f64) -> Player {
    let mut p = Player::new();
    p.x = x;
    p.y = y;
    p
}

pub(super) fn coin_at(x: f64, y: f64) -> Coin {
    Coin { x, y, collected: false }
}

pub(super) fn enemy_at(x: f64, y: f64) -> Enemy {
    Enemy { x, y, vx: -1.5, alive: true, squish_timer: 0 }
}
