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
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use game::GameState;
use input::InputState;

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

    // ── GameState ────────────────────────────────────────────────────────────
    let state = Rc::new(RefCell::new(GameState::new(ctx, Rc::clone(&input))));

    // ── Game loop ─────────────────────────────────────────────────────────────
    start_game_loop(state);

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
