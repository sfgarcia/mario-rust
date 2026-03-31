use crate::input::InputState;
use crate::world::{Tile, World, TILE_SIZE};

const GRAVITY: f64       = 0.55;
const MAX_FALL: f64      = 14.0;
const MOVE_SPEED: f64    = 4.2;
const JUMP_VEL: f64      = -11.5;
const FRICTION: f64      = 0.75; // desaceleración horizontal al soltar teclas

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub on_ground: bool,
    pub alive: bool,
    pub facing_right: bool,
    /// Frames desde último salto (para one-shot jump)
    pub jump_cooldown: u32,
}

impl Player {
    pub const WIDTH: f64  = 26.0;
    pub const HEIGHT: f64 = 32.0;

    pub fn new() -> Self {
        Player {
            x: 2.0 * TILE_SIZE,
            y: 11.0 * TILE_SIZE,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            alive: true,
            facing_right: true,
            jump_cooldown: 0,
        }
    }

    pub fn update(&mut self, input: &InputState, world: &World) {
        if !self.alive { return; }

        // ── Input horizontal ─────────────────────────────────────────────────
        if input.left {
            self.vx = -MOVE_SPEED;
            self.facing_right = false;
        } else if input.right {
            self.vx = MOVE_SPEED;
            self.facing_right = true;
        } else {
            self.vx *= FRICTION;
            if self.vx.abs() < 0.1 { self.vx = 0.0; }
        }

        // ── Salto ────────────────────────────────────────────────────────────
        if self.jump_cooldown > 0 { self.jump_cooldown -= 1; }
        if input.jump_pressed && self.on_ground && self.jump_cooldown == 0 {
            self.vy = JUMP_VEL;
            self.on_ground = false;
            self.jump_cooldown = 8;
        }

        // ── Gravedad ─────────────────────────────────────────────────────────
        self.vy = (self.vy + GRAVITY).min(MAX_FALL);

        // ── Movimiento X + colisión ──────────────────────────────────────────
        self.x += self.vx;
        self.x = self.resolve_x(world);

        // Límite izquierdo del nivel
        if self.x < 0.0 { self.x = 0.0; self.vx = 0.0; }

        // ── Movimiento Y + colisión ──────────────────────────────────────────
        self.on_ground = false;
        self.y += self.vy;
        self.resolve_y(world);

        // ── Caída al vacío ───────────────────────────────────────────────────
        let world_bottom = crate::world::LEVEL_ROWS as f64 * TILE_SIZE;
        if self.y > world_bottom + 64.0 {
            self.alive = false;
        }
    }

    fn resolve_x(&mut self, world: &World) -> f64 {
        let left   = self.x + 2.0;
        let right  = self.x + Player::WIDTH - 2.0;
        let top    = self.y + 2.0;
        let bottom = self.y + Player::HEIGHT - 2.0;
        let mid_y  = self.y + Player::HEIGHT / 2.0;

        if self.vx > 0.0 {
            // Moviendo a la derecha — comprobar borde derecho
            for py in &[top, mid_y, bottom] {
                let t = world.tile_at_px(right, *py);
                if World::is_solid(t) {
                    let col = (right / TILE_SIZE).floor() as i64;
                    let snapped = col as f64 * TILE_SIZE - Player::WIDTH - 2.0;
                    self.vx = 0.0;
                    return snapped;
                }
            }
        } else if self.vx < 0.0 {
            // Moviendo a la izquierda — comprobar borde izquierdo
            for py in &[top, mid_y, bottom] {
                let t = world.tile_at_px(left, *py);
                if World::is_solid(t) {
                    let col = (left / TILE_SIZE).floor() as i64;
                    let snapped = (col + 1) as f64 * TILE_SIZE - 2.0;
                    self.vx = 0.0;
                    return snapped;
                }
            }
        }
        self.x
    }

    fn resolve_y(&mut self, world: &World) {
        // Iteramos sobre todas las columnas que el jugador ocupa para no perder
        // tiles en los bordes (el check con 2 puntos fijos podía saltarse un tile
        // si el jugador solapaba 1-2 px de ladrillo en el borde).
        let col_first = (self.x / TILE_SIZE).floor().max(0.0) as usize;
        let col_last  = ((self.x + Player::WIDTH - 1.0) / TILE_SIZE).floor().max(0.0) as usize;

        let top    = self.y + 1.0;
        let bottom = self.y + Player::HEIGHT;

        if self.vy > 0.0 {
            // Cayendo — comprobar pies
            let row = (bottom / TILE_SIZE).floor() as i64;
            // Solo comprobamos filas dentro del nivel; fuera solo existe el vacío
            if row >= 0 && (row as usize) < crate::world::LEVEL_ROWS {
                for col in col_first..=col_last {
                    if World::is_solid_from_above(world.tile_at(col, row as usize)) {
                        self.y = row as f64 * TILE_SIZE - Player::HEIGHT;
                        self.vy = 0.0;
                        self.on_ground = true;
                        return;
                    }
                }
            }
        } else if self.vy < 0.0 {
            // Subiendo — comprobar cabeza
            let row = (top / TILE_SIZE).floor() as i64;
            if row >= 0 && (row as usize) < crate::world::LEVEL_ROWS {
                for col in col_first..=col_last {
                    let t = world.tile_at(col, row as usize);
                    if matches!(t, Tile::Ground | Tile::Brick | Tile::PipeBody | Tile::PipeCap) {
                        self.y = (row + 1) as f64 * TILE_SIZE;
                        self.vy = 0.0;
                        return;
                    }
                }
            }
        }
    }
}
