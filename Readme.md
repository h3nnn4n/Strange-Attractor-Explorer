# Strange Attractor Explorer

A simple strange attractor explorer written in Rust and Typescript. For now
only the Clifford Attractor is available. I plan to support entering custom
equations in the future.

<p float="left">
  <img src="/examples/example1.jpg" width="100" />
  <img src="/examples/example2.jpg" width="100" />
  <img src="/examples/example3.jpg" width="100" />
</p>

## How to use

The `a`, `b`, `c` and `d` parameters
defines the attractor. There are two sets of such parameters because the explorer
draws several strange attractors on top of each other. It starts with the values
from the first line, and interpolates it until it reaches the second set of
parameters. The number of points in the interpolation is set by `frames`. The
number of iterations that each attractors is iterated for a given set of
parameters is set by `iters`. Note that the total number of function evaluations
is `frames` times `iters`. Watch out for big numbers. It is possible to control
the image brightness by setting `gamma`. If you only want one image to be drawn,
that is, dont interpolate parameters, just set `frames` to 1.

## How to run

1. Clone this repo
2. Have the [rust toolchain](https://www.rust-lang.org/tools/install) installed
3. Have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed
4. Run `yarn install`. If you dont have it, you can get it [here](https://yarnpkg.com/lang/en/docs/install/)
5. Run `wasm-pack build --release`
6. Run `yarn link` inside the `pkg` folder
7. Run `yarn link "strange-attractor-explorer"` on the project root (this is the project name on the _config.toml_ file)
8. Run `yarn dev-server`
9. Go to `http://localhost:8080/`

## TODO

Some features that I want to implement in the nearby future:

- A progress bar
- Saving/downloading the results
- ~Colors~
  - A color picker
- A simple parser using reverse polish notation for custom attractors

## License

Check the _LICENSE_ file for more information.
