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

        let mut data = self.init_image();

        for _ in 0..self.iters {
            x = (self.a * yn).sin() + self.c * (self.a * xn).cos();
            y = (self.b * xn).sin() + self.d * (self.b * yn).cos();

            xn = x;
            yn = y;

            self.put_pixel(x, y, &mut data);
        }

        self.normalize_image(&data)
    }

    fn normalize_image(&self, data: &Vec<u64>) -> Vec<u8> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut normalized_data = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                if data[index + 0] > r {
                    r = data[index + 0];
                }

                if data[index + 1] > g {
                    g = data[index + 1];
                }

                if data[index + 2] > b {
                    b = data[index + 2];
                }
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                normalized_data.push(((data[index + 0] as f64 / r as f64) * 255 as f64) as u8);
                normalized_data.push(((data[index + 1] as f64 / g as f64) * 255 as f64) as u8);
                normalized_data.push(((data[index + 2] as f64 / b as f64) * 255 as f64) as u8);
                normalized_data.push(255);
            }
        }

        normalized_data
    }

    fn put_pixel(&self, x: f64, y: f64, data: &mut Vec<u64>) {
        let xi = (((x - self.minx) * self.width as f64) / (self.maxx - self.minx)) as u64;
        let yi = (((y - self.miny) * self.height as f64) / (self.maxy - self.miny)) as u64;

        let index = ((yi * self.width as u64 + xi) * 4) as usize;

        data[index + 0] += 1;
        data[index + 1] += 1;
        data[index + 2] += 1;
    }

    fn init_image(&self) -> Vec<u64> {
        let mut data = Vec::new();

        for _ in 0..self.width {
            for _ in 0..self.height {
                data.push(0);
                data.push(0);
                data.push(0);
                data.push(255);
            }
        }

        data
    }
}
