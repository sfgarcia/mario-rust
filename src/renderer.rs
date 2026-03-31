use web_sys::CanvasRenderingContext2d;

use crate::game::{GamePhase, GameState};
use crate::player::Player;
use crate::world::{Enemy, Tile, CANVAS_H, CANVAS_W, LEVEL_ROWS, TILE_SIZE};

pub fn render(ctx: &CanvasRenderingContext2d, state: &GameState) {
    let cx = state.camera_x;

    // ── Fondo ────────────────────────────────────────────────────────────────
    fill(ctx, "#5c94fc", 0.0, 0.0, CANVAS_W, CANVAS_H);

    // Nubes decorativas (estáticas, coordenadas de mundo)
    draw_clouds(ctx, cx);

    // ── Tiles ────────────────────────────────────────────────────────────────
    let first_col = (cx / TILE_SIZE).floor() as usize;
    let last_col  = ((cx + CANVAS_W) / TILE_SIZE).ceil() as usize + 1;

    for row in 0..LEVEL_ROWS {
        for col in first_col..last_col.min(crate::world::LEVEL_COLS) {
            let tile = state.world.tile_at(col, row);
            let sx = col as f64 * TILE_SIZE - cx;
            let sy = row as f64 * TILE_SIZE;
            draw_tile(ctx, tile, sx, sy);
        }
    }

    // ── Bandera ───────────────────────────────────────────────────────────────
    draw_flag(ctx, state.world.flag_x - cx);

    // ── Monedas ───────────────────────────────────────────────────────────────
    for coin in &state.world.coins {
        if !coin.collected {
            let sx = coin.x - cx;
            let sy = coin.y;
            if sx > -TILE_SIZE && sx < CANVAS_W + TILE_SIZE {
                draw_coin(ctx, sx, sy);
            }
        }
    }

    // ── Enemigos ──────────────────────────────────────────────────────────────
    for enemy in &state.world.enemies {
        let sx = enemy.x - cx;
        if sx > -48.0 && sx < CANVAS_W + 48.0 {
            draw_enemy(ctx, enemy, sx);
        }
    }

    // ── Jugador ───────────────────────────────────────────────────────────────
    if state.player.alive {
        let sx = state.player.x - cx;
        draw_player(ctx, &state.player, sx);
    }

    // ── HUD ───────────────────────────────────────────────────────────────────
    draw_hud(ctx, state);

    // ── Overlay muerte / victoria ─────────────────────────────────────────────
    match state.phase {
        GamePhase::Title   => draw_overlay(ctx, "Platform Adventure", "#2266cc", "Pulsa cualquier tecla para jugar"),
        GamePhase::Dead    => draw_overlay(ctx, "¡Perdiste!", "#cc2222", "Pulsa R para reiniciar"),
        GamePhase::Won     => draw_overlay(ctx, "¡Ganaste!",  "#22aa22", "Pulsa R para jugar de nuevo"),
        GamePhase::Playing => {}
    }
}

// ─── Helpers de color ────────────────────────────────────────────────────────

fn fill(ctx: &CanvasRenderingContext2d, color: &str, x: f64, y: f64, w: f64, h: f64) {
    ctx.set_fill_style_str(color);
    ctx.fill_rect(x, y, w, h);
}

// ─── Tiles ───────────────────────────────────────────────────────────────────

fn draw_tile(ctx: &CanvasRenderingContext2d, tile: Tile, sx: f64, sy: f64) {
    match tile {
        Tile::Empty => {}
        Tile::Ground => draw_ground(ctx, sx, sy),
        Tile::Brick  => draw_brick(ctx, sx, sy),
        Tile::PipeCap  => draw_pipe_cap(ctx, sx, sy),
        Tile::PipeBody => draw_pipe_body(ctx, sx, sy),
    }
}

fn draw_ground(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    // Cuerpo marrón
    fill(ctx, "#a05000", x, y, TILE_SIZE, TILE_SIZE);
    // Franja verde arriba (solo si el tile de arriba es vacío — se simplifica dibujando siempre)
    fill(ctx, "#56a000", x, y, TILE_SIZE, 6.0);
    // Líneas de tierra
    ctx.set_fill_style_str("#7b3800");
    ctx.fill_rect(x + 4.0, y + 8.0, TILE_SIZE - 8.0, 2.0);
    ctx.fill_rect(x + 4.0, y + 16.0, TILE_SIZE - 8.0, 2.0);
}

fn draw_brick(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    // Fondo ladrillo
    fill(ctx, "#c84800", x, y, TILE_SIZE, TILE_SIZE);
    // Líneas de mortero horizontales
    ctx.set_fill_style_str("#8b2e00");
    ctx.fill_rect(x, y + 10.0, TILE_SIZE, 2.0);
    ctx.fill_rect(x, y + 22.0, TILE_SIZE, 2.0);
    // Líneas verticales (alternadas)
    ctx.fill_rect(x + 16.0, y,       2.0, 10.0);
    ctx.fill_rect(x + 8.0,  y + 12.0, 2.0, 10.0);
    ctx.fill_rect(x + 24.0, y + 12.0, 2.0, 10.0);
    ctx.fill_rect(x + 16.0, y + 24.0, 2.0, 8.0);
    // Borde claro arriba
    fill(ctx, "#e06020", x, y, TILE_SIZE, 2.0);
}

fn draw_pipe_cap(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    // Tapa del tubo: más ancha que el cuerpo
    fill(ctx, "#00a800", x - 3.0, y, TILE_SIZE * 2.0 + 6.0, TILE_SIZE);
    // Borde claro arriba
    fill(ctx, "#00d800", x - 3.0, y, TILE_SIZE * 2.0 + 6.0, 4.0);
    // Borde oscuro abajo
    fill(ctx, "#006600", x - 3.0, y + TILE_SIZE - 4.0, TILE_SIZE * 2.0 + 6.0, 4.0);
}

fn draw_pipe_body(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    fill(ctx, "#00a800", x, y, TILE_SIZE * 2.0, TILE_SIZE);
    // Franja de luz
    fill(ctx, "#00c800", x + 4.0, y, 6.0, TILE_SIZE);
}

// ─── Bandera ─────────────────────────────────────────────────────────────────

fn draw_flag(ctx: &CanvasRenderingContext2d, sx: f64) {
    if sx < -40.0 || sx > CANVAS_W + 10.0 { return; }
    // Poste
    fill(ctx, "#888888", sx + 16.0, 7.0 * TILE_SIZE, 4.0, 7.0 * TILE_SIZE);
    // Bandera roja
    fill(ctx, "#ff2222", sx + 20.0, 7.0 * TILE_SIZE, 24.0, 16.0);
    // Bola en la cima
    ctx.set_fill_style_str("#ffdd00");
    ctx.begin_path();
    let _ = ctx.arc(sx + 18.0, 7.0 * TILE_SIZE - 1.0, 5.0, 0.0, std::f64::consts::TAU);
    ctx.fill();
}

// ─── Moneda ──────────────────────────────────────────────────────────────────

fn draw_coin(ctx: &CanvasRenderingContext2d, sx: f64, sy: f64) {
    ctx.set_fill_style_str("#ffd700");
    ctx.begin_path();
    let _ = ctx.arc(sx, sy, 7.0, 0.0, std::f64::consts::TAU);
    ctx.fill();
    // Brillo
    ctx.set_fill_style_str("#fff8a0");
    ctx.begin_path();
    let _ = ctx.arc(sx - 2.0, sy - 2.0, 3.0, 0.0, std::f64::consts::TAU);
    ctx.fill();
}

// ─── Enemigo (estilo goomba) ──────────────────────────────────────────────────

fn draw_enemy(ctx: &CanvasRenderingContext2d, enemy: &Enemy, sx: f64) {
    if !enemy.alive {
        if enemy.squish_timer > 0 {
            // Aplastado: rect horizontal
            fill(ctx, "#7b3f00", sx, enemy.y + Enemy::HEIGHT - 8.0, Enemy::WIDTH, 8.0);
        }
        return;
    }
    let x = sx;
    let y = enemy.y;
    let w = Enemy::WIDTH;
    let h = Enemy::HEIGHT;

    // Cuerpo
    fill(ctx, "#c84b00", x, y, w, h);
    // Pies (dos bloques oscuros abajo)
    fill(ctx, "#7b3f00", x, y + h - 8.0, w / 2.0 - 1.0, 8.0);
    fill(ctx, "#7b3f00", x + w / 2.0 + 1.0, y + h - 8.0, w / 2.0 - 1.0, 8.0);
    // Ojos blancos
    fill(ctx, "#ffffff", x + 4.0,  y + 4.0,  8.0, 8.0);
    fill(ctx, "#ffffff", x + w - 12.0, y + 4.0, 8.0, 8.0);
    // Pupilas
    fill(ctx, "#000000", x + 6.0,  y + 6.0,  4.0, 4.0);
    fill(ctx, "#000000", x + w - 10.0, y + 6.0, 4.0, 4.0);
    // Cejas fruncidas
    ctx.set_fill_style_str("#000000");
    ctx.fill_rect(x + 3.0, y + 3.0, 9.0, 2.0);
    ctx.fill_rect(x + w - 12.0, y + 3.0, 9.0, 2.0);
}

// ─── Jugador ──────────────────────────────────────────────────────────────────

fn draw_player(ctx: &CanvasRenderingContext2d, player: &Player, sx: f64) {
    let x = sx;
    let y = player.y;
    let w = Player::WIDTH;
    let h = Player::HEIGHT;

    // Usar el transform del canvas para el espejo — evita calcular posiciones a mano.
    ctx.save();
    if !player.facing_right {
        // Trasladar al centro del jugador, escalar -1 en X, volver
        let _ = ctx.translate(x + w / 2.0, 0.0);
        let _ = ctx.scale(-1.0, 1.0);
        let _ = ctx.translate(-(x + w / 2.0), 0.0);
    }

    // Todo se dibuja como si mirara a la derecha; el transform hace el espejo.
    let lx = x; // origen local

    // Gorra (roja)
    ctx.set_fill_style_str("#dd2222");
    ctx.fill_rect(lx + 2.0, y, w - 4.0, 10.0);
    ctx.fill_rect(lx - 2.0, y + 4.0, w + 4.0, 6.0);

    // Cara (piel)
    ctx.set_fill_style_str("#ffcc88");
    ctx.fill_rect(lx + 1.0, y + 10.0, w - 2.0, 10.0);

    // Bigote
    ctx.set_fill_style_str("#4a2800");
    ctx.fill_rect(lx + 3.0, y + 16.0, w - 6.0, 3.0);

    // Ojo (siempre en el lado derecho de la cara → frente del personaje)
    ctx.set_fill_style_str("#ffffff");
    ctx.fill_rect(lx + 14.0, y + 11.0, 6.0, 5.0);
    ctx.set_fill_style_str("#000000");
    ctx.fill_rect(lx + 15.0, y + 12.0, 4.0, 4.0);

    // Cuerpo (overol azul)
    ctx.set_fill_style_str("#2244cc");
    ctx.fill_rect(lx + 2.0, y + 20.0, w - 4.0, 8.0);

    // Botones overol
    ctx.set_fill_style_str("#ffdd00");
    ctx.fill_rect(lx + 5.0,      y + 21.0, 3.0, 3.0);
    ctx.fill_rect(lx + w - 8.0,  y + 21.0, 3.0, 3.0);

    // Piernas
    ctx.set_fill_style_str("#dd2222");
    ctx.fill_rect(lx + 2.0,      y + 28.0, 10.0, h - 28.0);
    ctx.fill_rect(lx + w - 12.0, y + 28.0, 10.0, h - 28.0);

    // Zapatos
    ctx.set_fill_style_str("#4a2800");
    ctx.fill_rect(lx,            y + h - 5.0, 12.0, 5.0);
    ctx.fill_rect(lx + w - 12.0, y + h - 5.0, 12.0, 5.0);

    // Brazos
    ctx.set_fill_style_str("#ffcc88");
    ctx.fill_rect(lx - 4.0,  y + 20.0, 6.0, 8.0);
    ctx.fill_rect(lx + w - 2.0, y + 20.0, 6.0, 8.0);

    ctx.restore();
}

// ─── HUD ─────────────────────────────────────────────────────────────────────

fn draw_hud(ctx: &CanvasRenderingContext2d, state: &GameState) {
    // Barra semitransparente
    ctx.set_fill_style_str("rgba(0,0,0,0.45)");
    ctx.fill_rect(0.0, 0.0, CANVAS_W, 28.0);

    ctx.set_fill_style_str("#ffffff");
    ctx.set_font("bold 16px monospace");

    let coins_collected = state.world.coins.iter().filter(|c| c.collected).count();
    let total_coins     = state.world.coins.len();

    let _ = ctx.fill_text("PLATFORM ADVENTURE", 10.0, 19.0);
    let score_text = format!("MONEDAS: {}/{}", coins_collected, total_coins);
    let _ = ctx.fill_text(&score_text, CANVAS_W - 200.0, 19.0);
}

// ─── Overlay ─────────────────────────────────────────────────────────────────

fn draw_overlay(
    ctx: &CanvasRenderingContext2d,
    title: &str,
    color: &str,
    subtitle: &str,
) {
    // Fondo semitransparente
    ctx.set_fill_style_str("rgba(0,0,0,0.65)");
    ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

    // Panel central
    let pw = 400.0;
    let ph = 160.0;
    let px = (CANVAS_W - pw) / 2.0;
    let py = (CANVAS_H - ph) / 2.0;
    ctx.set_fill_style_str("rgba(20,20,20,0.92)");
    ctx.fill_rect(px, py, pw, ph);
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(3.0);
    ctx.stroke_rect(px, py, pw, ph);

    // Título
    ctx.set_fill_style_str(color);
    ctx.set_font("bold 40px monospace");
    ctx.set_text_align("center");
    let _ = ctx.fill_text(title, CANVAS_W / 2.0, py + 70.0);

    // Subtítulo
    ctx.set_fill_style_str("#cccccc");
    ctx.set_font("18px monospace");
    let _ = ctx.fill_text(subtitle, CANVAS_W / 2.0, py + 110.0);

    // Resetear alineación
    ctx.set_text_align("left");
}

// ─── Nubes decorativas ───────────────────────────────────────────────────────

fn draw_clouds(ctx: &CanvasRenderingContext2d, camera_x: f64) {
    let cloud_positions: &[(f64, f64)] = &[
        (200.0, 60.0), (600.0, 40.0), (1100.0, 70.0),
        (1600.0, 50.0), (2200.0, 65.0), (2800.0, 45.0),
        (3400.0, 70.0), (4000.0, 55.0), (4600.0, 40.0),
        (5200.0, 68.0), (5800.0, 50.0), (6200.0, 60.0),
    ];
    for &(wx, wy) in cloud_positions {
        let sx = wx - camera_x * 0.4; // parallax suave
        if sx > -80.0 && sx < CANVAS_W + 80.0 {
            draw_cloud(ctx, sx, wy);
        }
    }
}

fn draw_cloud(ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
    ctx.set_fill_style_str("rgba(255,255,255,0.88)");
    for &(ox, oy, r) in &[(0.0, 0.0, 18.0f64), (22.0, -10.0, 22.0f64), (46.0, 0.0, 18.0f64), (22.0, 6.0, 24.0f64)] {
        ctx.begin_path();
        let _ = ctx.arc(x + ox, y + oy, r, 0.0, std::f64::consts::TAU);
        ctx.fill();
    }
}
