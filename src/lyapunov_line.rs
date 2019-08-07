use lyapunov::Lyapunov;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LyapunovLine {
    start: [f64; 4],
    end: [f64; 4],

    rng: rand::prelude::ThreadRng,
}

#[wasm_bindgen]
impl LyapunovLine {
    pub fn new() -> LyapunovLine {
        LyapunovLine {
            start: [0.0, 0.0, 0.0, 0.0],
            end: [0.0, 0.0, 0.0, 0.0],

            rng: rand::thread_rng(),
        }
    }

    pub fn set_start_parameters(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.start = [a, b, c, d];
    }

    pub fn set_end_parameters(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.end = [a, b, c, d];
    }

    pub fn find_chaotic_params(&mut self) {
        let mut lyap = Lyapunov::new();
        lyap.set_iters(100_000);
        let mut lyap2 = Lyapunov::new();

        'outer: for _ in 0..1_000 {
            lyap.find_chaotic_params();

            for _ in 0..1_000 {
                let d_a = self.rng.gen_range(-0.2, 0.2);
                let d_b = self.rng.gen_range(-0.2, 0.2);
                let d_c = self.rng.gen_range(-0.2, 0.2);
                let d_d = self.rng.gen_range(-0.2, 0.2);

                lyap2.set_parameters(lyap.a + d_a, lyap.b + d_b, lyap.c + d_c, lyap.d + d_d);
                lyap2.set_iters(100_000);
                lyap2.evaluate();

                if lyap2.is_chaotic() {
                    break 'outer;
                }
            }
        }

        self.start = [lyap.a, lyap.b, lyap.c, lyap.d];
        self.end = [lyap2.a, lyap2.b, lyap2.c, lyap2.d];
    }

    pub fn get_start_params(&self) -> Vec<f64> {
        self.start.to_vec()
    }

    pub fn get_end_params(&self) -> Vec<f64> {
        self.end.to_vec()
    }
}
