import * as $ from 'jquery';

var Rust: any;

const init = () => {
  Rust.init();
  Rust.render_attractor();

  bindEvents();
}

function bindEvents() {
  $('#render').click(renderButtonClickEvent);

  $('#a_value_end').change(renderButtonClickEvent);
  $('#b_value_end').change(renderButtonClickEvent);
  $('#c_value_end').change(renderButtonClickEvent);
  $('#d_value_end').change(renderButtonClickEvent);

  $('#a_value_start').change(renderButtonClickEvent);
  $('#b_value_start').change(renderButtonClickEvent);
  $('#c_value_start').change(renderButtonClickEvent);
  $('#d_value_start').change(renderButtonClickEvent);

  $('#gamma_value').change(renderButtonClickEvent);
  $('#iters_value').change(renderButtonClickEvent);
  $('#frames_value').change(renderButtonClickEvent);
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
