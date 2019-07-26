import * as $ from 'jquery';

var Rust: any;

const init = () => {
  Rust.init();
  Rust.render_attractor();

  $('#render').click(renderButtonClickEvent);
  $('#a_value').change(renderButtonClickEvent);
  $('#b_value').change(renderButtonClickEvent);
  $('#c_value').change(renderButtonClickEvent);
  $('#d_value').change(renderButtonClickEvent);
  $('#gamma_value').change(renderButtonClickEvent);
  $('#iters_value').change(renderButtonClickEvent);
}

function renderButtonClickEvent() {
  Rust.render_attractor();
}

export const load = () => {
  (() => import( /* webpackChunkName: "clifford_attractor" */ './pkg/clifford_attractor.js').then(module => {
    Rust = module;
    init();
  }))();
}
