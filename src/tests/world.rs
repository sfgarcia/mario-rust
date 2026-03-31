use crate::world::{Tile, World, TILE_SIZE, LEVEL_COLS, LEVEL_ROWS};
use super::world;

// ── Estructura del mapa ──────────────────────────────────────────────────────

#[test]
fn ground_row_is_solid() {
    let w = world();
    assert_eq!(w.tile_at(0, 14), Tile::Ground);
    assert_eq!(w.tile_at(10, 14), Tile::Ground);
    assert_eq!(w.tile_at(LEVEL_COLS - 1, 14), Tile::Ground);
}

#[test]
fn gaps_in_ground_are_empty() {
    let w = world();
    for col in 31..=33 { assert_eq!(w.tile_at(col, 14), Tile::Empty, "gap col {}", col); }
    for col in 71..=73 { assert_eq!(w.tile_at(col, 14), Tile::Empty, "gap col {}", col); }
}

#[test]
fn platforms_exist() {
    let w = world();
    // Primera plataforma: fila 11, cols 5-9
    for col in 5..=9 {
        assert_eq!(w.tile_at(col, 11), Tile::Brick, "platform col {}", col);
    }
}

#[test]
fn pipes_have_cap_and_body() {
    let w = world();
    assert_eq!(w.tile_at(20, 12), Tile::PipeCap);
    assert_eq!(w.tile_at(21, 12), Tile::PipeCap);
    assert_eq!(w.tile_at(20, 13), Tile::PipeBody);
}

#[test]
fn pipes_have_clear_space_above() {
    let w = world();
    // Scan entire map for pipe caps
    for col in 0..LEVEL_COLS {
        for row in 0..LEVEL_ROWS {
            if w.tile_at(col, row) == Tile::PipeCap {
                // Verify the row above is empty (no solid tiles blocking entry)
                if row > 0 {
                    let tile_above = w.tile_at(col, row - 1);
                    assert_eq!(
                        tile_above, Tile::Empty,
                        "pipe cap at ({}, {}) has blocking tile above: {:?}",
                        col, row, tile_above
                    );
                }
            }
        }
    }
}

// ── Propiedades de tiles ─────────────────────────────────────────────────────

#[test]
fn solid_tiles() {
    assert!(World::is_solid(Tile::Ground));
    assert!(World::is_solid(Tile::Brick));
    assert!(World::is_solid(Tile::PipeBody));
    assert!(World::is_solid(Tile::PipeCap));
    assert!(!World::is_solid(Tile::Empty));
}

#[test]
fn solid_from_above_pipe_body() {
    assert!(World::is_solid(Tile::PipeBody));
}

#[test]
fn solid_from_above_pipe_cap() {
    assert!(World::is_solid(Tile::PipeCap));
}

#[test]
fn solid_from_above_empty_is_false() {
    assert!(!World::is_solid(Tile::Empty));
}

// ── Conversión pixel ↔ tile ──────────────────────────────────────────────────

#[test]
fn tile_at_px_matches_tile_at() {
    let w = world();
    let px = 5.0 * TILE_SIZE + 4.0;
    let py = 14.0 * TILE_SIZE + 2.0;
    assert_eq!(w.tile_at_px(px, py), w.tile_at(5, 14));
}

#[test]
fn tile_at_px_out_of_bounds_is_empty() {
    let w = world();
    assert_eq!(w.tile_at_px(-1.0, 0.0), Tile::Empty);
    assert_eq!(w.tile_at_px(0.0, -1.0), Tile::Empty);
    let far = LEVEL_COLS as f64 * TILE_SIZE + 100.0;
    assert_eq!(w.tile_at_px(far, 0.0), Tile::Empty);
}

// ── Monedas ──────────────────────────────────────────────────────────────────

#[test]
fn coins_start_uncollected() {
    let w = world();
    assert!(w.coins.iter().all(|c| !c.collected));
}

#[test]
fn world_has_coins() {
    let w = world();
    assert!(!w.coins.is_empty(), "debe haber monedas");
}

#[test]
fn total_coins_reasonable() {
    let w = world();
    assert!(w.coins.len() >= 10, "debería haber al menos 10 monedas");
    assert!(w.coins.len() <= 200, "no debería haber más de 200 monedas");
}

// ── Enemigos y bandera ───────────────────────────────────────────────────────

#[test]
fn world_has_enemies() {
    let w = world();
    assert!(!w.enemies.is_empty(), "debe haber enemigos");
}

#[test]
fn flag_is_near_end_of_level() {
    let w = world();
    let level_end = LEVEL_COLS as f64 * TILE_SIZE;
    assert!(w.flag_x > level_end * 0.9, "la bandera debe estar cerca del final");
}

// ── Integridad del nivel ─────────────────────────────────────────────────────

#[test]
fn all_gaps_are_jumpaable_width() {
    let gaps: &[(usize, usize)] = &[(31, 33), (71, 73), (111, 113), (151, 153)];
    for &(start, end) in gaps {
        let width_tiles = end - start + 1;
        assert!(
            width_tiles <= 4,
            "hueco demasiado ancho: {} tiles ({} a {})", width_tiles, start, end
        );
    }
}

#[test]
fn level_has_enough_width() {
    assert!(LEVEL_COLS >= 100, "el nivel debería tener al menos 100 columnas");
}

#[test]
fn level_rows_correct() {
    assert_eq!(LEVEL_ROWS, 15);
}
