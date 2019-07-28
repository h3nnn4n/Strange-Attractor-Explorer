import { bindEvents, readValue, getCanvas } from './js/user_interface';
import { CliffordManager } from './js/clifford_manager';

var Rust: any;

const init = () => {
  Rust.init();

  bindEvents(render_attractor);

  render_attractor();
}

function render_attractor() {
  let clifford_manager = new CliffordManager(Rust);

  clifford_manager.interpolate_and_render();
  clifford_manager.draw_to_canvas();
}

export const load = () => {
  (() => import( /* webpackChunkName: "strange_attractor_explorer" */ './pkg/strange_attractor_explorer.js').then(module => {
    Rust = module;
    init();
  }))();
}
