import { bindEvents } from './js/user_interface';
import { CliffordManager } from './js/clifford_manager';

var Rust: any;
var clifford_manager: CliffordManager;

const init = () => {
  Rust.init();
  clifford_manager = new CliffordManager(Rust);

  bindEvents(render_attractor);

  render_attractor();
}

function render_attractor() {
  clifford_manager.start();

  render_loop();
}

function render_loop() {
  if (clifford_manager.finished_running()) {
    return;
  }

  clifford_manager.interpolate_and_render_step()

  setTimeout(() => {
    render_loop();
  }, 0);
}

export const load = () => {
  (() => import( /* webpackChunkName: "strange_attractor_explorer" */ './pkg/strange_attractor_explorer.js').then(module => {
    Rust = module;
    init();
  }))();
}
