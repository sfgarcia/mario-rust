use crate::world::{Enemy, TILE_SIZE};
use super::world;

// ── Movimiento y timers ──────────────────────────────────────────────────────

#[test]
fn enemies_move_horizontally() {
    let mut w = world();
    let x0 = w.enemies[0].x;
    w.update_enemies();
    let x1 = w.enemies[0].x;
    assert_ne!(x0, x1, "el enemigo debería moverse");
}

#[test]
fn dead_enemy_squish_timer_decrements() {
    let mut w = world();
    w.enemies[0].alive = false;
    w.enemies[0].squish_timer = 5;
    w.update_enemies();
    assert_eq!(w.enemies[0].squish_timer, 4);
}

#[test]
fn dead_enemy_squish_timer_stops_at_zero() {
    let mut w = world();
    w.enemies[0].alive = false;
    w.enemies[0].squish_timer = 0;
    for _ in 0..10 { w.update_enemies(); }
    assert_eq!(w.enemies[0].squish_timer, 0);
}

// ── IA: pared y precipicio ───────────────────────────────────────────────────

#[test]
fn enemy_bounces_off_wall() {
    let mut w = world();
    // El primer tubo está en col 20 (x=640). Colocar enemigo moviéndose hacia él.
    let pipe_x = 20.0 * TILE_SIZE;
    w.enemies[0].x = pipe_x - Enemy::WIDTH - 2.0;
    w.enemies[0].y = 12.0 * TILE_SIZE;
    w.enemies[0].vx = 1.5; // hacia la derecha (hacia el tubo)
    w.update_enemies();
    assert!(w.enemies[0].vx < 0.0, "el enemigo debe rebotar al chocar con el tubo");
}

#[test]
fn enemy_reverses_at_cliff() {
    let mut w = world();
    // Primer hueco en cols 31-33. Enemigo en col 30 moviéndose hacia el hueco.
    let cliff_x = 30.0 * TILE_SIZE + Enemy::WIDTH / 2.0;
    w.enemies[0].x = cliff_x;
    w.enemies[0].y = 13.0 * TILE_SIZE - Enemy::HEIGHT;
    w.enemies[0].vx = 1.5; // hacia la derecha (hacia el hueco)
    let vx_before = w.enemies[0].vx;
    w.update_enemies();
    assert!(
        w.enemies[0].vx != vx_before || w.enemies[0].vx < 0.0,
        "el enemigo debe invertir dirección al borde del precipicio"
    );
}
