use std::cell::RefCell;
use std::rc::Rc;

use web_sys::CanvasRenderingContext2d;

use crate::input::InputState;
use crate::player::Player;
use crate::world::{Coin, Enemy, World, CANVAS_W, LEVEL_COLS, TILE_SIZE};

// ── Funciones puras extraídas de step() ─────────────────────────────────────
// Permiten testear la lógica del juego sin depender de CanvasRenderingContext2d.

pub(crate) fn collect_coins(player: &Player, coins: &mut [Coin]) {
    let px = player.x;
    let py = player.y;
    let pw = Player::WIDTH;
    let ph = Player::HEIGHT;
    for coin in coins.iter_mut() {
        if coin.collected { continue; }
        let cx = coin.x - Coin::RADIUS;
        let cy = coin.y - Coin::RADIUS;
        if px < cx + Coin::RADIUS * 2.0 && px + pw > cx && py < cy + Coin::RADIUS * 2.0 && py + ph > cy {
            coin.collected = true;
        }
    }
}

pub(crate) fn resolve_enemy_interactions(player: &mut Player, enemies: &mut [Enemy]) {
    let px = player.x;
    let py = player.y;
    let pw = Player::WIDTH;
    let ph = Player::HEIGHT;
    for enemy in enemies.iter_mut() {
        if !enemy.alive { continue; }
        let overlap = px < enemy.x + Enemy::WIDTH
            && px + pw > enemy.x
            && py < enemy.y + Enemy::HEIGHT
            && py + ph > enemy.y;
        if overlap {
            let stomping = player.vy > 0.5
                && (py + ph) <= enemy.y + Enemy::HEIGHT / 2.0;
            if stomping {
                enemy.alive = false;
                enemy.squish_timer = 20;
                player.vy = -8.0;
            } else {
                player.alive = false;
            }
        }
    }
}

pub(crate) fn check_win(player: &Player, flag_x: f64) -> bool {
    player.x + Player::WIDTH > flag_x
}

pub(crate) fn compute_camera(player_x: f64) -> f64 {
    let target = player_x - CANVAS_W / 2.0 + Player::WIDTH / 2.0;
    let max_cam = LEVEL_COLS as f64 * TILE_SIZE - CANVAS_W;
    target.max(0.0).min(max_cam)
}

// ────────────────────────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
pub enum GamePhase {
    Title,
    Playing,
    Dead,
    Won,
}

pub struct GameState {
    pub player: Player,
    pub world: World,
    pub input: Rc<RefCell<InputState>>,
    pub camera_x: f64,
    pub phase: GamePhase,
    pub ctx: CanvasRenderingContext2d,
    last_timestamp: f64,
}

impl GameState {
    pub fn new(ctx: CanvasRenderingContext2d, input: Rc<RefCell<InputState>>) -> Self {
        GameState {
            player: Player::new(),
            world: World::new(),
            input,
            camera_x: 0.0,
            phase: GamePhase::Title,
            ctx,
            last_timestamp: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.player = Player::new();
        self.world = World::new();
        self.camera_x = 0.0;
        self.phase = GamePhase::Playing;
        self.last_timestamp = 0.0;
    }

    pub fn update(&mut self, timestamp: f64) {
        // ── Delta time ───────────────────────────────────────────────────────
        let dt = if self.last_timestamp == 0.0 {
            16.667
        } else {
            (timestamp - self.last_timestamp).min(50.0)
        };
        self.last_timestamp = timestamp;
        // Normalizamos a 60fps para que la física no dependa del framerate
        let steps = (dt / 16.667).round().max(1.0) as u32;

        // ── Title screen: cualquier tecla de movimiento arranca el juego ─────
        if self.phase == GamePhase::Title {
            let inp = self.input.borrow();
            if inp.left || inp.right || inp.jump_pressed || inp.jump {
                drop(inp);
                self.phase = GamePhase::Playing;
            }
            return;
        }

        // ── Restart ──────────────────────────────────────────────────────────
        let restart_pressed = self.input.borrow().restart_pressed;
        if restart_pressed {
            self.input.borrow_mut().restart_pressed = false;
            if self.phase != GamePhase::Playing {
                self.reset();
                return;
            }
        }

        if self.phase != GamePhase::Playing {
            return;
        }

        for _ in 0..steps {
            self.step();
        }
    }

    fn step(&mut self) {
        let input_snapshot = self.input.borrow().clone();
        self.input.borrow_mut().jump_pressed = false;
        step_logic(&mut self.player, &mut self.world, &input_snapshot, &mut self.camera_x, &mut self.phase);
    }
}

/// Ejecuta un tick de lógica sin depender de CanvasRenderingContext2d.
/// Usado tanto por GameState::step() como por GameSim::tick() en tests.
pub(crate) fn step_logic(
    player: &mut crate::player::Player,
    world: &mut crate::world::World,
    input: &InputState,
    camera_x: &mut f64,
    phase: &mut GamePhase,
) {
    if *phase != GamePhase::Playing { return; }
    player.update(input, world);
    world.update_enemies();
    collect_coins(player, &mut world.coins);
    resolve_enemy_interactions(player, &mut world.enemies);
    if check_win(player, world.flag_x) { *phase = GamePhase::Won; }
    if !player.alive { *phase = GamePhase::Dead; }
    *camera_x = compute_camera(player.x);
}

