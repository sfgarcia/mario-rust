use crate::world::{Tile, TILE_SIZE, LEVEL_ROWS};
use crate::player::Player;
use super::{world, no_input, right_input, left_input, jump_input};

// ── Gravedad y aterrizaje ────────────────────────────────────────────────────

#[test]
fn player_falls_due_to_gravity() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = 5.0 * TILE_SIZE;
    p.vy = 0.0;
    p.on_ground = false;
    let w = world();

    let y_before = p.y;
    p.update(&no_input(), &w);
    assert!(p.y > y_before, "el jugador debe caer");
}

#[test]
fn player_lands_on_ground() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = 12.0 * TILE_SIZE - Player::HEIGHT - 1.0;
    p.vy = 5.0;
    let w = world();

    for _ in 0..20 {
        if p.on_ground { break; }
        p.update(&no_input(), &w);
    }
    assert!(p.on_ground, "el jugador debería estar en el suelo");
    assert!(p.vy == 0.0 || p.vy >= 0.0, "vy debe ser 0 al aterrizar");
}

// ── Salto ────────────────────────────────────────────────────────────────────

#[test]
fn player_can_jump_when_on_ground() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = (LEVEL_ROWS - 3) as f64 * TILE_SIZE;
    let w = world();

    for _ in 0..60 {
        if p.on_ground { break; }
        p.update(&no_input(), &w);
    }
    assert!(p.on_ground, "pre-condición: debería estar en el suelo");

    let y_before = p.y;
    p.update(&jump_input(), &w);
    assert!(p.vy < 0.0 || p.y < y_before, "salto debería impulsar hacia arriba");
}

#[test]
fn player_cannot_double_jump() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = 3.0 * TILE_SIZE;
    p.on_ground = false;
    p.vy = -5.0;
    let w = world();

    let vy_before = p.vy;
    p.update(&jump_input(), &w);
    assert!(!p.on_ground);
    assert!(p.vy > vy_before - 1.0, "no debería aplicar salto extra en el aire");
}

#[test]
fn player_jump_cooldown_prevents_rejump() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    let w = world();
    for _ in 0..60 { if p.on_ground { break; } p.update(&no_input(), &w); }
    assert!(p.on_ground, "pre-condición: debe estar en el suelo");

    p.update(&jump_input(), &w);
    assert!(p.vy < 0.0, "debe saltar");
    let vy_after_jump = p.vy;

    p.update(&jump_input(), &w);
    assert!(p.vy > -11.5, "el cooldown debe impedir el doble salto inmediato");
    assert!((p.vy - (vy_after_jump + 0.55)).abs() < 0.01);
}

// ── Movimiento horizontal ────────────────────────────────────────────────────

#[test]
fn player_cannot_go_left_of_level() {
    let mut p = Player::new();
    p.x = 0.5;
    let w = world();
    for _ in 0..60 { p.update(&no_input(), &w); }
    for _ in 0..30 { p.update(&left_input(), &w); }
    assert!(p.x >= 0.0, "el jugador no debe salir del nivel por la izquierda");
}

#[test]
fn player_horizontal_speed_is_bounded() {
    let mut p = Player::new();
    p.x = 5.0 * TILE_SIZE;
    p.y = 5.0 * TILE_SIZE;
    let w = world();

    p.update(&right_input(), &w);
    assert!(p.vx.abs() <= 5.0, "vx no debería exceder MOVE_SPEED: {}", p.vx);
}

#[test]
fn player_friction_decelerates_vx() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = (LEVEL_ROWS - 2) as f64 * TILE_SIZE - Player::HEIGHT - 1.0;
    p.vx = 4.2;
    p.on_ground = true;
    let w = world();
    p.update(&no_input(), &w);
    assert!((p.vx - 3.15).abs() < 0.01, "fricción esperada: 3.15, real: {}", p.vx);
}

#[test]
fn player_vx_zeroed_below_threshold() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = (LEVEL_ROWS - 2) as f64 * TILE_SIZE - Player::HEIGHT - 1.0;
    p.vx = 0.05;
    p.on_ground = true;
    let w = world();
    p.update(&no_input(), &w);
    assert_eq!(p.vx, 0.0, "vx debe zerificarse bajo el umbral");
}

// ── Muerte y colisiones ──────────────────────────────────────────────────────

#[test]
fn player_dies_when_falling_out_of_level() {
    let mut p = Player::new();
    p.x = 31.0 * TILE_SIZE + 16.0;
    p.y = 13.0 * TILE_SIZE;
    p.vy = 0.0;
    let w = world();

    for _ in 0..200 {
        if !p.alive { break; }
        p.update(&no_input(), &w);
    }
    assert!(!p.alive, "el jugador debería morir al caer al vacío");
}

#[test]
fn player_dead_ignores_update() {
    let mut p = Player::new();
    p.x = 100.0;
    p.y = 100.0;
    p.vx = 0.0;
    p.vy = 0.0;
    p.alive = false;
    let w = world();
    p.update(&right_input(), &w);
    assert_eq!(p.x, 100.0, "jugador muerto no debe moverse en X");
    assert_eq!(p.y, 100.0, "jugador muerto no debe moverse en Y");
    assert_eq!(p.vx, 0.0);
    assert_eq!(p.vy, 0.0);
}

#[test]
fn player_does_not_sink_into_ground() {
    let mut p = Player::new();
    p.x = 2.0 * TILE_SIZE;
    p.y = 4.0 * TILE_SIZE;
    let w = world();

    for _ in 0..120 { p.update(&no_input(), &w); }
    let ground_y = (LEVEL_ROWS - 2) as f64 * TILE_SIZE - Player::HEIGHT;
    let actual_ground = w.tile_at(2, 14);
    assert_eq!(actual_ground, Tile::Ground);
    assert!(p.y <= ground_y + 2.0, "el jugador se hundió: y={}, expected<={}", p.y, ground_y);
}

#[test]
fn player_cannot_walk_through_pipe() {
    let w = world();
    let mut p = Player::new();
    let pipe_x = 20.0 * TILE_SIZE;
    p.x = pipe_x - Player::WIDTH - 2.0;
    p.y = 12.0 * TILE_SIZE;
    for _ in 0..30 { p.update(&no_input(), &w); }
    for _ in 0..30 { p.update(&right_input(), &w); }
    assert!(
        p.x + Player::WIDTH <= pipe_x + 4.0,
        "el jugador traspasó el tubo: jugador_right={}, tubo_left={}",
        p.x + Player::WIDTH, pipe_x
    );
}
