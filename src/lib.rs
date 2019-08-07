extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod clifford;
mod image;
mod lyapunov;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use clifford::Clifford;
use image::Image;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn init_image_data(width: u32, height: u32) -> Image {
    Image::init(width.into(), height.into())
}

#[wasm_bindgen]
pub fn init_attractor() -> Clifford {
    Clifford::new()
}

#[wasm_bindgen]
pub fn init() {
    utils::console_log("init");
    let canvas = utils::get_canvas();

    canvas.set_width(600);
    canvas.set_height(600);
}
