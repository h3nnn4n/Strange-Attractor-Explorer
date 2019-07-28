import { bindEvents, readValue, getCanvas } from './js/user_interface';
import { Clifford } from './pkg/strange_attractor_explorer';

var Rust: any;

const init = () => {
  Rust.init();

  bindEvents(render_attractor);

  render_attractor();
}

function render_attractor() {
  let canvas = getCanvas();
  let width = canvas.width;
  let height = canvas.height;

  let attractor = Rust.init_attractor();
  let image_data = Rust.init_image_data(width, height);

  let iters = readValue('iters_value');
  let frames = readValue('frames_value');
  let gamma = readValue('gamma_value');

  let parameters_start = [
    readValue('a_value_start'),
    readValue('b_value_start'),
    readValue('c_value_start'),
    readValue('d_value_start'),
  ];

  let parameters_end = [
    readValue('a_value_end'),
    readValue('b_value_end'),
    readValue('c_value_end'),
    readValue('d_value_end'),
  ];

  attractor.set_iters(iters);

  interpolate_and_render(attractor, image_data, parameters_start, parameters_end, frames);
  draw_to_canvas(image_data, gamma, width, height);
}

function interpolate_and_render(attractor: Clifford, image_data: any, parameters_start: number[], parameters_end: number[], frames: number) {
  for (let mode = 0; mode < 2; mode++) {
    for (let frame = 0; frame < frames; frame++) {
      let p = frame / frames;

      let parameters = interpolate_parameters(parameters_start, parameters_end, p)
      set_attractor_params(attractor, parameters);

      if (mode == 0) {
        attractor.find_bounding_box(false);
      } else if (mode == 1) {
        attractor.iterate(image_data);
      }
    }
  }
}

function interpolate_parameters(parameters_start: number[], parameters_end: number[], p: number): number[] {
  let parameters = [
    0, 0, 0, 0
  ];

  for (let index = 0; index < parameters_start.length; index++) {
    parameters[index] =
      (parameters_end[index] - parameters_start[index]) * p + parameters_start[index];
  }

  return parameters;
}

function set_attractor_params(attractor: Clifford, parameters: number[]) {
  attractor.set_parameters(
    parameters[0],
    parameters[1],
    parameters[2],
    parameters[3],
  );
}

function draw_to_canvas(image_data: any, gamma: number, width: number, height: number) {
  image_data.normalize_image();
  image_data.invert_colors();
  image_data.gamma_correction(gamma);

  let image_pixels = image_data.as_u8();

  let canvas = getCanvas();
  let canvas_context = canvas.getContext('2d');

  if (canvas_context != null) {
    let clamped_array = new Uint8ClampedArray(image_pixels);
    let image_data = new ImageData(clamped_array, width, height);

    canvas_context.putImageData(image_data, 0, 0);
  }
}

export const load = () => {
  (() => import( /* webpackChunkName: "strange_attractor_explorer" */ './pkg/strange_attractor_explorer.js').then(module => {
    Rust = module;
    init();
  }))();
}
