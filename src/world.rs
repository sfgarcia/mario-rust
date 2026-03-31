pub const TILE_SIZE: f64 = 32.0;
const ENEMY_GRAVITY: f64 = 4.0;
pub const LEVEL_COLS: usize = 210;
pub const LEVEL_ROWS: usize = 15;

/// Canvas lógico
pub const CANVAS_W: f64 = 800.0;
pub const CANVAS_H: f64 = 480.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Ground,
    Brick,    // plataforma de ladrillo
    PipeBody, // cuerpo de tubo
    PipeCap,  // tapa del tubo (fila superior)
}

pub struct Coin {
    pub x: f64,
    pub y: f64,
    pub collected: bool,
}

impl Coin {
    pub const RADIUS: f64 = 8.0;
}

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub alive: bool,
    /// Contador de frames para animación aplastamiento
    pub squish_timer: u32,
}

impl Enemy {
    pub fn new(x: f64, row: usize) -> Self {
        Enemy {
            x,
            y: row as f64 * TILE_SIZE,
            vx: -1.5,
            alive: true,
            squish_timer: 0,
        }
    }

    pub const WIDTH: f64 = 28.0;
    pub const HEIGHT: f64 = 28.0;
}

pub struct World {
    pub tiles: Box<[[Tile; LEVEL_COLS]; LEVEL_ROWS]>,
    pub coins: Vec<Coin>,
    pub enemies: Vec<Enemy>,
    pub flag_x: f64,
}

impl World {
    pub fn new() -> Self {
        let mut tiles = Box::new([[Tile::Empty; LEVEL_COLS]; LEVEL_ROWS]);

        // ── Suelo base (fila 14) ──────────────────────────────────────────────
        // Huecos en: cols 31-33, 71-73, 111-113, 151-153
        let gaps: &[(usize, usize)] = &[(31, 33), (71, 73), (111, 113), (151, 153)];
        for col in 0..LEVEL_COLS {
            let in_gap = gaps.iter().any(|&(s, e)| col >= s && col <= e);
            if !in_gap {
                tiles[14][col] = Tile::Ground;
                tiles[13][col] = Tile::Ground; // capa de relleno bajo la superficie
            }
        }

        // ── Plataformas flotantes ─────────────────────────────────────────────
        let platforms: &[(usize, usize, usize)] = &[
            // (fila, col_inicio, col_fin)
            (11, 5,  9),
            (10, 15, 19),
            (9,  24, 27),
            (10, 35, 39),
            (9,  45, 48),
            (11, 55, 60),
            (10, 65, 69),
            (9,  80, 84),
            (10, 93, 97),
            (9,  105, 109),
            (11, 120, 125),
            (10, 133, 137),
            (9,  143, 147),
            (11, 160, 165),
            (10, 173, 177),
            (9,  190, 194),
            (8,  195, 199),
        ];
        for &(row, c0, c1) in platforms {
            for col in c0..=c1 {
                tiles[row][col] = Tile::Brick;
            }
        }

        // ── Tubos ─────────────────────────────────────────────────────────────
        // formato: (col_izq, fila_tapa, altura_en_tiles)
        let pipes: &[(usize, usize, usize)] = &[
            (20, 12, 3),
            (43, 11, 4),
            (60, 10, 5),
            (90, 11, 4),
            (125, 10, 5),
            (160, 11, 4),
            (185, 10, 5),
        ];
        for &(col, cap_row, height) in pipes {
            // tapa (2 columnas de ancho)
            tiles[cap_row][col]     = Tile::PipeCap;
            tiles[cap_row][col + 1] = Tile::PipeCap;
            // cuerpo
            for r in (cap_row + 1)..(cap_row + height) {
                tiles[r][col]     = Tile::PipeBody;
                tiles[r][col + 1] = Tile::PipeBody;
            }
        }

        // ── Monedas ───────────────────────────────────────────────────────────
        let mut coins: Vec<Coin> = Vec::new();

        // Monedas en arco sobre plataformas
        let coin_groups: &[(usize, usize, usize)] = &[
            // (col_inicio, col_fin, fila)
            (5,  9,  10),
            (15, 19,  9),
            (24, 27,  8),
            (35, 39,  9),
            (45, 48,  8),
            (55, 60, 9),
            (65, 69,  9),
            (80, 84,  8),
            (93, 97,  9),
            (105,109,  8),
            (120,125, 9),
            (133,137,  9),
            (143,147,  8),
            (160,165, 10),
            (173,177,  9),
            (185,189,  8),
        ];
        for &(c0, c1, row) in coin_groups {
            for col in c0..=c1 {
                coins.push(Coin {
                    x: col as f64 * TILE_SIZE + TILE_SIZE / 2.0,
                    y: row as f64 * TILE_SIZE + TILE_SIZE / 2.0,
                    collected: false,
                });
            }
        }

        // ── Enemigos ──────────────────────────────────────────────────────────
        // Los ponemos encima del suelo (fila 12 = y de los pies)
        let enemies = vec![
            Enemy::new(14.0 * TILE_SIZE, 12),
            Enemy::new(25.0 * TILE_SIZE, 12),
            Enemy::new(37.0 * TILE_SIZE, 12),
            Enemy::new(57.0 * TILE_SIZE, 12),
            Enemy::new(75.0 * TILE_SIZE, 12),
            Enemy::new(95.0 * TILE_SIZE, 12),
            Enemy::new(115.0 * TILE_SIZE, 12),
            Enemy::new(140.0 * TILE_SIZE, 12),
            Enemy::new(165.0 * TILE_SIZE, 12),
            Enemy::new(190.0 * TILE_SIZE, 12),
        ];

        World {
            tiles,
            coins,
            enemies,
            flag_x: 200.0 * TILE_SIZE,
        }
    }

    pub fn tile_at(&self, col: usize, row: usize) -> Tile {
        if col < LEVEL_COLS && row < LEVEL_ROWS {
            self.tiles[row][col]
        } else {
            Tile::Ground // borde del mundo = sólido
        }
    }

    pub fn tile_at_px(&self, px: f64, py: f64) -> Tile {
        let col = (px / TILE_SIZE).floor() as i64;
        let row = (py / TILE_SIZE).floor() as i64;
        if col < 0 || row < 0 || col >= LEVEL_COLS as i64 || row >= LEVEL_ROWS as i64 {
            return Tile::Empty;
        }
        self.tiles[row as usize][col as usize]
    }

    pub fn is_solid(tile: Tile) -> bool {
        matches!(tile, Tile::Ground | Tile::Brick | Tile::PipeBody | Tile::PipeCap)
    }

    /// Actualiza la lógica de los enemigos.
    /// Itera por índice para poder tomar préstamos inmutables de `self.tiles`
    /// mientras mutamos cada `self.enemies[i]`.
    pub fn update_enemies(&mut self) {
        for i in 0..self.enemies.len() {
            // Bajar el squish timer de los muertos
            if !self.enemies[i].alive {
                if self.enemies[i].squish_timer > 0 {
                    self.enemies[i].squish_timer -= 1;
                }
                continue;
            }

            // Mover horizontalmente
            self.enemies[i].x += self.enemies[i].vx;

            let feet_y  = self.enemies[i].y + Enemy::HEIGHT + 1.0;
            let left_x  = self.enemies[i].x + 2.0;
            let right_x = self.enemies[i].x + Enemy::WIDTH - 2.0;
            let mid_y   = self.enemies[i].y + Enemy::HEIGHT / 2.0;
            let vx      = self.enemies[i].vx;

            // Comprobar pared adelante
            let front_x    = if vx < 0.0 { left_x } else { right_x };
            let front_tile = self.tile_at_px(front_x, mid_y);
            if World::is_solid(front_tile) {
                self.enemies[i].vx = -vx;
                self.enemies[i].x += -vx * 2.0;
            }

            // No caminar en el vacío
            let vx2         = self.enemies[i].vx;
            let ground_x    = if vx2 < 0.0 { left_x } else { right_x };
            let ground_tile = self.tile_at_px(ground_x, feet_y);
            if !World::is_solid(ground_tile) {
                self.enemies[i].vx = -vx2;
            }

            // Gravedad
            let on_ground = World::is_solid(self.tile_at_px(left_x, feet_y))
                         || World::is_solid(self.tile_at_px(right_x, feet_y));
            if !on_ground {
                self.enemies[i].y += ENEMY_GRAVITY;
            } else {
                let row = ((self.enemies[i].y + Enemy::HEIGHT) / TILE_SIZE).floor() as usize;
                self.enemies[i].y = row as f64 * TILE_SIZE - Enemy::HEIGHT;
            }
        }
    }
}
