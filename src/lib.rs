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

use clifford::Clifford;
use image_data::ImageData;

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

    let a_parameter_start = utils::get_input_value_by_id("a_value_start");
    let b_parameter_start = utils::get_input_value_by_id("b_value_start");
    let c_parameter_start = utils::get_input_value_by_id("c_value_start");
    let d_parameter_start = utils::get_input_value_by_id("d_value_start");

    let a_parameter_end = utils::get_input_value_by_id("a_value_end");
    let b_parameter_end = utils::get_input_value_by_id("b_value_end");
    let c_parameter_end = utils::get_input_value_by_id("c_value_end");
    let d_parameter_end = utils::get_input_value_by_id("d_value_end");

    let gamma_parameter = utils::get_input_value_by_id("gamma_value");
    let iters_parameter = utils::get_input_value_by_id("iters_value") as u64;
    let frames_parameter = utils::get_input_value_by_id("frames_value") as u64;

    let width = canvas.width();
    let height = canvas.height();

    let mut attractor = Clifford::new();
    let mut img_data = ImageData::init(width.into(), height.into());

    attractor.set_iters(iters_parameter);

    for mode in 0..2 {
        for frame_count in 0..frames_parameter {
            let p = frame_count as f64 / frames_parameter as f64;

            let a_parameter = (a_parameter_end - a_parameter_start) * p + a_parameter_start;
            let b_parameter = (b_parameter_end - b_parameter_start) * p + b_parameter_start;
            let c_parameter = (c_parameter_end - c_parameter_start) * p + c_parameter_start;
            let d_parameter = (d_parameter_end - d_parameter_start) * p + d_parameter_start;

            attractor.set_parameters(a_parameter, b_parameter, c_parameter, d_parameter);

            if mode == 0 {
                attractor.find_bounding_box(false);
            } else if mode == 1 {
                attractor.iterate(&mut img_data);
            }
        }

        if mode == 0 {
            attractor.bump_bounding_box();
        }
    }

    utils::console_log("found bounding box");

    img_data.normalize_image();
    img_data.invert_colors();
    img_data.gamma_correction(gamma_parameter);

    let mut data = img_data.as_u8();

    let data =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)
            .unwrap();

    context.put_image_data(&data, 0.0, 0.0).unwrap();
    utils::console_log("finished rendering");
}
