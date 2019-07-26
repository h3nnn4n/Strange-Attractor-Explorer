extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod clifford;
mod image_data;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn init() {
    utils::console_log("init");
    let canvas = utils::get_canvas();

    canvas.set_width(600);
    canvas.set_height(600);
}

#[wasm_bindgen]
pub fn render_attractor() {
    utils::console_log("started rendering");
    let canvas = utils::get_canvas();
    let context = utils::get_canvas_context();

    let a_parameter = utils::get_input_value_by_id("a_value");
    let b_parameter = utils::get_input_value_by_id("b_value");
    let c_parameter = utils::get_input_value_by_id("c_value");
    let d_parameter = utils::get_input_value_by_id("d_value");

    let gamma_parameter = utils::get_input_value_by_id("gamma_value");
    let iters_parameter = utils::get_input_value_by_id("iters_value");

    let width = canvas.width();
    let height = canvas.height();

    let mut attractor = clifford::Clifford::new();

    attractor.set_parameters(a_parameter, b_parameter, c_parameter, d_parameter);
    attractor.set_iters(iters_parameter as u64);

    let mut raw_data = attractor.iterate();

    raw_data.normalize_image();
    raw_data.invert_colors();
    raw_data.gamma_correction(gamma_parameter);

    let mut data = raw_data.as_u8();

    let data =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)
            .unwrap();

    context.put_image_data(&data, 0.0, 0.0).unwrap();
    utils::console_log("finished rendering");
}
