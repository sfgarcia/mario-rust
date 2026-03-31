# Platform Adventure 🎮

Juego de plataformas estilo Mario hecho en **Rust → WebAssembly**, con gráficos 100% programáticos (sin sprites externos).

**[▶ Jugar en el navegador](https://sfgarcia.github.io/mario-rust/)**

![Tests](https://github.com/sfgarcia/mario-rust/actions/workflows/test.yml/badge.svg)

---

## Controles

| Tecla | Acción |
|-------|--------|
| `←` / `A` | Mover izquierda |
| `→` / `D` | Mover derecha |
| `↑` / `Space` / `Z` | Saltar |
| `R` | Reiniciar |

---

## Stack

- **Rust** → lógica del juego (física, colisiones, IA de enemigos)
- **WebAssembly** (via `wasm-pack`) → compilación para el navegador
- **HTML5 Canvas 2D** → renderizado programático
- **GitHub Pages** → hosting gratuito

## Estructura

```
src/
├── lib.rs        # Entry point WASM, game loop con requestAnimationFrame
├── game.rs       # GameState, update, colisiones
├── world.rs      # Tiles (15×210), enemigos, monedas
├── player.rs     # Física AABB del jugador
├── renderer.rs   # Dibujado en Canvas 2D
├── sim.rs        # Simulador headless para tests
└── tests/        # 52 unit tests de lógica pura
tests/
└── game.spec.ts  # 7 tests Playwright e2e
```

## Build local

```bash
# Requisitos: rustup, wasm-pack
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --out-dir pkg --release
python -m http.server 8080
# Abrir http://localhost:8080
```

## Tests

```bash
# Unit tests (Rust, sin navegador)
cargo test

# E2E tests (Playwright, requiere build previo)
npm install
npx playwright install chromium
npx playwright test
```

## CI/CD

Cada push a `main` ejecuta automáticamente:
1. `cargo test` — 52 unit tests de lógica
2. `playwright test` — 7 tests e2e en Chromium headless
3. Deploy a GitHub Pages (solo en push a `main`)
