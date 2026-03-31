//! Simulador headless — solo compilado en builds de test.
//! Proporciona GameSim: equivalente de GameState sin CanvasRenderingContext2d.

use crate::game::{step_logic, GamePhase};
use crate::input::InputState;
use crate::player::Player;
use crate::world::World;

pub struct GameSim {
    pub player: Player,
    pub world: World,
    pub camera_x: f64,
    pub phase: GamePhase,
    /// Frames totales ejecutados desde creación o último reset.
    pub frame: u64,
}

impl GameSim {
    pub fn new() -> Self {
        GameSim {
            player: Player::new(),
            world: World::new(),
            camera_x: 0.0,
            phase: GamePhase::Playing,
            frame: 0,
        }
    }

    pub fn reset(&mut self) {
        self.player = Player::new();
        self.world = World::new();
        self.camera_x = 0.0;
        self.phase = GamePhase::Playing;
        self.frame = 0;
    }

    /// Avanza exactamente un tick con el input dado.
    /// `jump_pressed` se consume (se pone a false) tras este tick,
    /// igual que hace GameState::step() en producción.
    pub fn tick(&mut self, input: &mut InputState) {
        if self.phase != GamePhase::Playing {
            return;
        }
        let snapshot = input.clone();
        input.jump_pressed = false; // consumir el flanco, igual que el loop real
        step_logic(
            &mut self.player,
            &mut self.world,
            &snapshot,
            &mut self.camera_x,
            &mut self.phase,
        );
        self.frame += 1;
    }

    /// Avanza `frames` ticks con el mismo InputState.
    /// Si `jump_pressed` está a true solo se usa en el primer tick.
    pub fn run(&mut self, mut input: InputState, frames: u32) {
        for _ in 0..frames {
            if self.phase != GamePhase::Playing {
                break;
            }
            self.tick(&mut input);
        }
    }

    /// Ejecuta una secuencia de (input, frames) en orden.
    /// Se detiene antes si la fase deja de ser Playing.
    pub fn run_sequence(&mut self, seq: &[(InputState, u32)]) {
        for (input, frames) in seq {
            self.run(input.clone(), *frames);
            if self.phase != GamePhase::Playing {
                break;
            }
        }
    }

    // ── Helpers de estado ────────────────────────────────────────────────────

    pub fn coins_collected(&self) -> usize {
        self.world.coins.iter().filter(|c| c.collected).count()
    }

    pub fn enemies_alive(&self) -> usize {
        self.world.enemies.iter().filter(|e| e.alive).count()
    }

    pub fn is_playing(&self) -> bool { self.phase == GamePhase::Playing }
    pub fn is_dead(&self)    -> bool { self.phase == GamePhase::Dead    }
    pub fn is_won(&self)     -> bool { self.phase == GamePhase::Won     }
}
