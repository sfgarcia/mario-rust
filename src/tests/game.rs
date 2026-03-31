use crate::player::Player;
use crate::world::{Coin, Enemy, TILE_SIZE, LEVEL_COLS, CANVAS_W};
use crate::game::{collect_coins, resolve_enemy_interactions, check_win, compute_camera};
use super::{player_at, coin_at, enemy_at};

// ── Recogida de monedas ──────────────────────────────────────────────────────

#[test]
fn collect_coin_when_player_overlaps() {
    let player = player_at(100.0, 100.0);
    let mut coins = vec![coin_at(player.x + 8.0, player.y + 8.0)];
    collect_coins(&player, &mut coins);
    assert!(coins[0].collected);
}

#[test]
fn no_coin_collected_when_far() {
    let player = player_at(0.0, 0.0);
    let mut coins = vec![coin_at(500.0, 500.0)];
    collect_coins(&player, &mut coins);
    assert!(!coins[0].collected);
}

#[test]
fn already_collected_coin_skipped() {
    let player = player_at(100.0, 100.0);
    let mut coins = vec![Coin { x: player.x + 8.0, y: player.y + 8.0, collected: true }];
    collect_coins(&player, &mut coins);
    assert!(coins[0].collected);
}

#[test]
fn coin_collected_at_right_edge() {
    // El borde derecho del jugador (x+26) solapa 1px el AABB de la moneda.
    let player = player_at(0.0, 0.0);
    // coin AABB izq = 33-8 = 25 → jugador right=26 > 25 → overlap de 1px
    let mut coins = vec![coin_at(33.0, 8.0)];
    collect_coins(&player, &mut coins);
    assert!(coins[0].collected, "debe recoger con overlap de 1px por la derecha");
}

// ── Interacción jugador-enemigo ──────────────────────────────────────────────

#[test]
fn stomp_kills_enemy_and_bounces_player() {
    let enemy_y = 200.0;
    let mut player = player_at(200.0, enemy_y + 5.0 - Player::HEIGHT);
    player.vy = 2.0;
    let mut enemies = vec![enemy_at(200.0, enemy_y)];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(!enemies[0].alive, "el enemigo debe morir");
    assert_eq!(enemies[0].squish_timer, 20);
    assert_eq!(player.vy, -8.0, "el jugador debe rebotar");
    assert!(player.alive, "el jugador debe sobrevivir al pisotón");
}

#[test]
fn stomp_at_exact_threshold() {
    let enemy_y = 200.0;
    let mut player = player_at(200.0, enemy_y + 10.0 - Player::HEIGHT);
    player.vy = 2.0;
    let mut enemies = vec![enemy_at(200.0, enemy_y)];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(!enemies[0].alive, "umbral exacto debe ser stomp");
}

#[test]
fn stomp_outside_threshold_kills_player() {
    // Umbral de stomp = enemy.y + Enemy::HEIGHT/2 = enemy_y + 14.
    // Con py+ph = enemy_y + 15 el jugador ya está demasiado dentro → no es stomp.
    let enemy_y = 200.0;
    let mut player = player_at(200.0, enemy_y + 15.0 - Player::HEIGHT);
    player.vy = 2.0;
    let mut enemies = vec![enemy_at(200.0, enemy_y)];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(!player.alive, "el jugador debe morir por colisión lateral");
    assert!(enemies[0].alive, "el enemigo debe sobrevivir");
}

#[test]
fn stomp_at_max_fall_speed() {
    // Regression: con MAX_FALL=14 el jugador puede entrar 14px en un frame.
    // El umbral antiguo (10px) lo mataba; el nuevo (HEIGHT/2=14px) lo deja pisotear.
    let enemy_y = 200.0;
    let mut player = player_at(200.0, enemy_y + 14.0 - Player::HEIGHT);
    player.vy = 14.0; // velocidad máxima de caída
    let mut enemies = vec![enemy_at(200.0, enemy_y)];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(!enemies[0].alive, "stomp a velocidad máxima debe matar al enemigo");
    assert!(player.alive, "el jugador debe sobrevivir al pisotón rápido");
}

#[test]
fn side_collision_kills_player() {
    let mut player = player_at(200.0, 200.0);
    player.vy = 0.0;
    let mut enemies = vec![enemy_at(210.0, 200.0)];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(!player.alive, "colisión lateral mata al jugador");
    assert!(enemies[0].alive, "el enemigo sobrevive a la colisión lateral");
}

#[test]
fn dead_enemy_not_checked_for_collision() {
    let mut player = player_at(200.0, 200.0);
    player.vy = 0.0;
    let mut enemies = vec![Enemy { x: 210.0, y: 200.0, vx: -1.5, alive: false, squish_timer: 5 }];
    resolve_enemy_interactions(&mut player, &mut enemies);
    assert!(player.alive, "enemigo muerto no debe matar al jugador");
}

// ── Victoria ─────────────────────────────────────────────────────────────────

#[test]
fn check_win_returns_true_past_flag() {
    let mut player = Player::new();
    let flag_x = 1000.0;
    player.x = flag_x - Player::WIDTH + 1.0;
    assert!(check_win(&player, flag_x));
}

#[test]
fn check_win_returns_false_before_flag() {
    let mut player = Player::new();
    let flag_x = 1000.0;
    player.x = flag_x - Player::WIDTH - 1.0;
    assert!(!check_win(&player, flag_x));
}

// ── Cámara ───────────────────────────────────────────────────────────────────

#[test]
fn camera_clamps_at_zero() {
    assert_eq!(compute_camera(0.0), 0.0);
    assert_eq!(compute_camera(50.0), 0.0);
}

#[test]
fn camera_clamps_at_max() {
    let max_cam = LEVEL_COLS as f64 * TILE_SIZE - CANVAS_W;
    assert_eq!(compute_camera(99_999.0), max_cam);
}

#[test]
fn camera_centers_on_player() {
    let expected = 2000.0 - CANVAS_W / 2.0 + Player::WIDTH / 2.0;
    let result = compute_camera(2000.0);
    assert!((result - expected).abs() < f64::EPSILON, "cámara={result}, esperado={expected}");
}
