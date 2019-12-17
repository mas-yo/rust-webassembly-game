use std::f64;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

mod components;
mod systems;
mod world;

pub(crate) type EntityID = u64;
pub(crate) type CanvasContext = web_sys::CanvasRenderingContext2d;

#[derive(Default)]
pub(crate) struct IdGenerator<T> {
    current: T,
}

impl<T> IdGenerator<T>
where
    T: num::traits::Num + Copy,
{
    fn new() -> Self {
        Self { current: T::one() }
    }
    fn next(&mut self) -> T {
        self.current = self.current.add(T::one());
        self.current
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("application started"));

    start()
}

use world::*;

#[derive(Clone)]
pub(crate) struct WindowEnv {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,
    pub canvas_context: web_sys::CanvasRenderingContext2d,
}

impl WindowEnv {
    fn create() -> Result<WindowEnv, JsValue> {
        let win = web_sys::window().ok_or(JsValue::from_str("no window"))?;
        let doc = win.document().ok_or(JsValue::from_str("no document"))?;
        let can = doc.get_element_by_id("canvas").ok_or(JsValue::from_str("no canvas"))?.dyn_into::<web_sys::HtmlCanvasElement>().or(Err(JsValue::from_str("cast error")))?;
        let con = can.get_context("2d")?.ok_or(JsValue::from_str("no 2d context"))?.dyn_into::<web_sys::CanvasRenderingContext2d>().or(Err(JsValue::from_str("cast error")))?;

        Ok(Self{
            window: win, document:doc, canvas:can, canvas_context:con
        })
    }

    fn request_animation_frame(&self, f: &Closure<dyn FnMut()>) -> Result<i32, JsValue> {
        self.window.request_animation_frame(f.as_ref().unchecked_ref())
    }
}

pub fn start() -> Result<(), JsValue> {

    let win_env = WindowEnv::create()?;

    let mut world = World::new(win_env.clone());
    // // world.update();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let env = win_env.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            env.canvas_context.clear_rect(0f64, 0f64, env.canvas.width() as f64, env.canvas.height() as f64);
            world.update();

            env.request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        }) as Box<dyn FnMut()>));
    }
    win_env.request_animation_frame(g.borrow().as_ref().unwrap())?;

    Ok(())
}
