use rand::Rng;

pub struct Lyapunov {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,

    pub lyapunov: f64,

    iters: u64,

    rng: rand::prelude::ThreadRng,
}

impl Lyapunov {
    pub fn new() -> Lyapunov {
        Lyapunov {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,

            lyapunov: 0.0,

            iters: 100_000,

            rng: rand::thread_rng(),
        }
    }

    pub fn set_parameters(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }

    pub fn set_iters(&mut self, iters: u32) {
        self.iters = iters.into();
    }

    pub fn evaluate(&mut self) {
        self.iterate();
    }

    pub fn find_chaotic_params(&mut self) {
        self.set_iters(100_000);

        loop {
            let a = self.rng.gen_range(-2.0, 2.0);
            let b = self.rng.gen_range(-2.0, 2.0);
            let c = self.rng.gen_range(-2.0, 2.0);
            let d = self.rng.gen_range(-2.0, 2.0);

            self.set_parameters(a, b, c, d);
            self.evaluate();

            if self.is_chaotic() {
                break;
            }
        }
    }

    pub fn is_chaotic(&self) -> bool {
        if self.lyapunov > 10.0 {
            return true;
        }

        false
    }

    fn iterate(&mut self) {
        let mut x;
        let mut y;
        let mut xn: f64 = self.rng.gen_range(-1.0, 1.0);
        let mut yn: f64 = self.rng.gen_range(-1.0, 1.0);

        let mut x_2;
        let mut y_2;
        let mut xn_2: f64 = xn + self.rng.gen_range(-0.25, 0.25);
        let mut yn_2: f64 = yn + self.rng.gen_range(-0.25, 0.25);

        let d_0 = self.distance(xn, yn, xn_2, yn_2);

        for i in 0..self.iters {
            x = (self.a * yn).sin() + self.c * (self.a * xn).cos();
            y = (self.b * xn).sin() + self.d * (self.b * yn).cos();
            x_2 = (self.a * yn_2).sin() + self.c * (self.a * xn_2).cos();
            y_2 = (self.b * xn_2).sin() + self.d * (self.b * yn_2).cos();

            xn = x;
            yn = y;
            xn_2 = x_2;
            yn_2 = y_2;

            if self.point_attractor(x, y, x_2, y_2) || self.infinity_attractor(x, y, x_2, y_2) {
                return;
            }

            if i > 10_000 {
                let d_d = self.distance(x, y, x_2, y_2);
                self.lyapunov += (d_d / d_0).abs().log2();
            }
        }
    }

    fn distance(&self, x: f64, y: f64, x_2: f64, y_2: f64) -> f64 {
        let dx = x - x_2;
        let dy = y - y_2;

        let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        dist
    }

    fn point_attractor(&self, x: f64, y: f64, x_2: f64, y_2: f64) -> bool {
        let limit = 0.000_001;

        let dx = x - x_2;
        let dy = y - y_2;

        let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        if dist < limit {
            return true;
        }

        false
    }

    fn infinity_attractor(&self, x: f64, y: f64, x_2: f64, y_2: f64) -> bool {
        let limit = 100_000.0;

        if y > limit || y < -limit || y_2 > limit || y_2 < -limit {
            return true;
        }

        if x > limit || x < -limit || x_2 > limit || x_2 < -limit {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chaotic_parameters() {
        let mut lyap = Lyapunov::new();

        lyap.set_iters(100_000);
        lyap.set_parameters(-1.4, 1.8, 1.1, 0.7);
        lyap.evaluate();

        assert!(lyap.is_chaotic());
    }

    #[test]
    fn test_point_parameters() {
        let mut lyap = Lyapunov::new();

        lyap.set_iters(100_000);
        lyap.set_parameters(1.0, 1.0, 1.0, 1.0);
        lyap.evaluate();

        assert!(!lyap.is_chaotic());
    }

    #[test]
    fn test_find_chaotic_parameters() {
        let mut lyap = Lyapunov::new();

        lyap.find_chaotic_params();

        assert!(lyap.is_chaotic());
    }
}
