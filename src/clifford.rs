use image_data;

pub struct Clifford {
    a: f64,
    b: f64,
    c: f64,
    d: f64,

    iters: u64,

    width: u64,
    height: u64,

    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
}

impl Clifford {
    pub fn new() -> Clifford {
        Clifford {
            a: -1.4,
            b: 1.6,
            c: 1.0,
            d: 0.7,

            iters: 10_000_000,

            width: 600,
            height: 600,

            minx: -2.0,
            miny: -2.0,
            maxx: 2.0,
            maxy: 2.0,
        }
    }

    pub fn iterate(&self) -> Vec<u8> {
        let mut x;
        let mut y;
        let mut xn = 0.5;
        let mut yn = 0.5;

        let mut data = image_data::ImageData::init(self.width, self.height);

        for _ in 0..self.iters {
            x = (self.a * yn).sin() + self.c * (self.a * xn).cos();
            y = (self.b * xn).sin() + self.d * (self.b * yn).cos();

            xn = x;
            yn = y;

            data.put_pixel(self.get_pixel_position(x, y));
        }

        data.normalize_image()
    }

    fn get_pixel_position(&self, x: f64, y: f64) -> (u64, u64) {
        let xi = (((x - self.minx) * self.width as f64) / (self.maxx - self.minx)) as u64;
        let yi = (((y - self.miny) * self.height as f64) / (self.maxy - self.miny)) as u64;

        (xi, yi)
    }
}
