use image_data::ImageData;

pub struct Clifford {
    a: f64,
    b: f64,
    c: f64,
    d: f64,

    iters: u64,

    config: Config,
}

pub struct Config {
    width: u64,
    height: u64,

    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 600,
            height: 600,

            minx: -2.0,
            miny: -2.0,
            maxx: 2.0,
            maxy: 2.0,
        }
    }
}

impl Clifford {
    pub fn new() -> Clifford {
        Clifford {
            a: -1.4,
            b: 1.6,
            c: 1.0,
            d: 0.7,

            iters: 1_000_000,

            config: Config::new(),
        }
    }

    pub fn set_parameters(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }

    pub fn set_iters(&mut self, iters: u64) {
        self.iters = iters;
    }

    pub fn iterate(&self) -> ImageData {
        let mut x;
        let mut y;
        let mut xn = 0.5;
        let mut yn = 0.5;

        let mut data = ImageData::init(self.config.width, self.config.height);

        for _ in 0..self.iters {
            x = (self.a * yn).sin() + self.c * (self.a * xn).cos();
            y = (self.b * xn).sin() + self.d * (self.b * yn).cos();

            xn = x;
            yn = y;

            data.put_pixel(self.get_pixel_position(x, y));
        }

        data
    }

    pub fn find_bounding_box(&mut self) {
        let mut x;
        let mut y;
        let mut xn = 0.5;
        let mut yn = 0.5;

        self.config.minx = 10.0;
        self.config.miny = 10.0;
        self.config.maxx = -10.0;
        self.config.maxy = -10.0;

        for _ in 0..10_000 {
            x = (self.a * yn).sin() + self.c * (self.a * xn).cos();
            y = (self.b * xn).sin() + self.d * (self.b * yn).cos();

            xn = x;
            yn = y;

            if x < self.config.minx {
                self.config.minx = x
            }

            if y < self.config.miny {
                self.config.miny = y
            }

            if x > self.config.maxx {
                self.config.maxx = x
            }

            if y > self.config.maxy {
                self.config.maxy = y
            }
        }

        self.config.minx *= 1.10;
        self.config.miny *= 1.10;
        self.config.maxx *= 1.10;
        self.config.maxy *= 1.10;
    }

    fn get_pixel_position(&self, x: f64, y: f64) -> (u64, u64) {
        let xi = (((x - self.config.minx) * self.config.width as f64)
            / (self.config.maxx - self.config.minx)) as u64;
        let yi = (((y - self.config.miny) * self.config.height as f64)
            / (self.config.maxy - self.config.miny)) as u64;

        (xi, yi)
    }
}
