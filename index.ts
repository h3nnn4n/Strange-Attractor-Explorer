var Rust: any;

const init = () => {
  Rust.init();
}

export const load = () => {
  (() => import( /* webpackChunkName: "clifford_attractor" */ './pkg/clifford_attractor.js').then(module => {
    Rust = module;
    init();
  }))();
}