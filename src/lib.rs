mod game;
mod input;
mod player;
mod renderer;
mod world;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod sim;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, TouchEvent};

use game::GameState;
use input::InputState;

// ── Touch button regions (canvas coordinates 800×480) ────────────────────────
const TOUCH_LEFT:  [f64; 4] = [20.0,  390.0, 70.0, 70.0]; // [x, y, w, h]
const TOUCH_RIGHT: [f64; 4] = [100.0, 390.0, 70.0, 70.0];
const TOUCH_JUMP:  [f64; 4] = [710.0, 390.0, 70.0, 70.0];

fn zone_hit(zone: [f64; 4], tx: f64, ty: f64) -> bool {
    tx >= zone[0] && tx < zone[0] + zone[2] && ty >= zone[1] && ty < zone[1] + zone[3]
}

fn process_touches(
    event: &TouchEvent,
    input: &mut InputState,
    canvas_w: f64,
    canvas_h: f64,
    rect_x: f64,
    rect_y: f64,
    rect_w: f64,
    rect_h: f64,
) {
    input.left  = false;
    input.right = false;

    let scale_x = canvas_w / rect_w;
    let scale_y = canvas_h / rect_h;

    let touches = event.touches();
    let count = touches.length();

    let mut any_touch = false;
    let mut jump_zone = false;

    for i in 0..count {
        if let Some(t) = touches.get(i) {
            let tx = (t.client_x() as f64 - rect_x) * scale_x;
            let ty = (t.client_y() as f64 - rect_y) * scale_y;
            any_touch = true;
            if zone_hit(TOUCH_LEFT,  tx, ty) { input.left  = true; }
            if zone_hit(TOUCH_RIGHT, tx, ty) { input.right = true; }
            if zone_hit(TOUCH_JUMP,  tx, ty) { jump_zone   = true; }
        }
    }

    if any_touch {
        input.restart_pressed = true;
    }

    // Rising-edge for jump; any touch also covers Title "press any key" detection
    let new_jump = jump_zone || any_touch;
    if new_jump && !input.jump {
        input.jump_pressed = true;
    }
    input.jump = new_jump;
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window   = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    // ── Canvas ───────────────────────────────────────────────────────────────
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("game-canvas")
        .expect("no #game-canvas")
        .dyn_into()?;

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .expect("no 2d context")
        .dyn_into()?;

    // ── InputState compartido ─────────────────────────────────────────────────
    let input = Rc::new(RefCell::new(InputState::default()));

    // ── Keyboard events ───────────────────────────────────────────────────────
    register_keyboard_events(&window, Rc::clone(&input))?;

    // ── Touch events ──────────────────────────────────────────────────────────
    register_touch_events(&canvas, Rc::clone(&input))?;

    // ── GameState ────────────────────────────────────────────────────────────
    let state = Rc::new(RefCell::new(GameState::new(ctx, Rc::clone(&input))));

    // ── Game loop ─────────────────────────────────────────────────────────────
    start_game_loop(state);

    Ok(())
}

fn register_touch_events(
    canvas: &HtmlCanvasElement,
    input: Rc<RefCell<InputState>>,
) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;

    macro_rules! touch_closure {
        ($input:expr, $canvas:expr) => {{
            let input   = Rc::clone(&$input);
            let canvas2 = $canvas.clone();
            Closure::<dyn FnMut(TouchEvent)>::wrap(Box::new(move |e: TouchEvent| {
                e.prevent_default();
                let rect = canvas2.get_bounding_client_rect();
                process_touches(
                    &e, &mut input.borrow_mut(),
                    800.0, 480.0,
                    rect.x(), rect.y(), rect.width(), rect.height(),
                );
            }))
        }};
    }

    let c = touch_closure!(input, canvas);
    canvas.add_event_listener_with_callback("touchstart", c.as_ref().unchecked_ref())?;
    c.forget();

    let c = touch_closure!(input, canvas);
    canvas.add_event_listener_with_callback("touchend", c.as_ref().unchecked_ref())?;
    c.forget();

    let c = touch_closure!(input, canvas);
    canvas.add_event_listener_with_callback("touchcancel", c.as_ref().unchecked_ref())?;
    c.forget();

    Ok(())
}

fn register_keyboard_events(
    window: &web_sys::Window,
    input: Rc<RefCell<InputState>>,
) -> Result<(), JsValue> {
    // keydown
    {
        let input = Rc::clone(&input);
        let closure = Closure::<dyn FnMut(KeyboardEvent)>::wrap(Box::new(
            move |e: KeyboardEvent| {
                let key = e.key();
                let mut s = input.borrow_mut();
                match key.as_str() {
                    "ArrowLeft"  | "a" | "A" => s.left  = true,
                    "ArrowRight" | "d" | "D" => s.right = true,
                    "ArrowUp" | " " | "w" | "W" | "z" | "Z" => {
                        if !s.jump {
                            s.jump_pressed = true;
                        }
                        s.jump = true;
                    }
                    "r" | "R" => {
                        if !s.restart {
                            s.restart_pressed = true;
                        }
                        s.restart = true;
                    }
                    _ => {}
                }
            },
        ));
        window.add_event_listener_with_callback(
            "keydown",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // keyup
    {
        let input = Rc::clone(&input);
        let closure = Closure::<dyn FnMut(KeyboardEvent)>::wrap(Box::new(
            move |e: KeyboardEvent| {
                let key = e.key();
                let mut s = input.borrow_mut();
                match key.as_str() {
                    "ArrowLeft"  | "a" | "A" => s.left  = false,
                    "ArrowRight" | "d" | "D" => s.right = false,
                    "ArrowUp" | " " | "w" | "W" | "z" | "Z" => s.jump = false,
                    "r" | "R" => s.restart = false,
                    _ => {}
                }
            },
        ));
        window.add_event_listener_with_callback(
            "keyup",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    Ok(())
}

fn start_game_loop(state: Rc<RefCell<GameState>>) {
    // Patrón clásico para requestAnimationFrame recursivo en Rust WASM:
    // La closure necesita referenciarse a sí misma, así que usamos
    // Rc<RefCell<Option<Closure>>> que se llena después de crearse.
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        // 1. Actualizar lógica
        state.borrow_mut().update(timestamp);

        // 2. Renderizar
        {
            let s = state.borrow();
            renderer::render(&s.ctx, &s);
        }

        // 3. Pedir el siguiente frame
        web_sys::window()
            .unwrap()
            .request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .unwrap();
    }) as Box<dyn FnMut(f64)>));

    // Arrancar el loop
    web_sys::window()
        .unwrap()
        .request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )
        .unwrap();
}
