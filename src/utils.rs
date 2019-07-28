#![allow(dead_code)]

extern crate web_sys;

// use self::web_sys::console;
use cfg_if::cfg_if;
use wasm_bindgen::JsCast;
use web_sys::console;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[macro_export]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn console_log(text: &str) {
    console::log_1(&text.into());
}

pub fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

pub fn get_canvas() -> web_sys::HtmlCanvasElement {
    let canvas: web_sys::HtmlCanvasElement = get_document()
        .get_element_by_id("canvas")
        .expect("canvas with id #canvas was not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}

pub fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
