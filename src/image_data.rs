pub struct ImageData {
    data: Vec<u64>,
    width: u64,
    height: u64,
}

impl ImageData {
    pub fn init(width: u64, height: u64) -> ImageData {
        let mut image_data = ImageData {
            data: vec![],
            width: width,
            height: height,
        };

        image_data.init_image();

        image_data
    }

    fn init_image(&mut self) {
        self.data.clear();

        for _ in 0..self.width {
            for _ in 0..self.height {
                self.data.push(0);
                self.data.push(0);
                self.data.push(0);
                self.data.push(255);
            }
        }
    }

    pub fn normalize_image(&mut self) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                if self.data[index + 0] > r {
                    r = self.data[index + 0];
                }

                if self.data[index + 1] > g {
                    g = self.data[index + 1];
                }

                if self.data[index + 2] > b {
                    b = self.data[index + 2];
                }
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                self.data[index + 0] =
                    ((self.data[index + 0] as f64 / r as f64) * 255 as f64) as u64;
                self.data[index + 1] =
                    ((self.data[index + 1] as f64 / g as f64) * 255 as f64) as u64;
                self.data[index + 2] =
                    ((self.data[index + 2] as f64 / b as f64) * 255 as f64) as u64;
            }
        }
    }

    pub fn put_pixel(&mut self, (x, y): (u64, u64)) {
        let index = ((y * self.width as u64 + x) * 4) as usize;

        self.data[index + 0] += 1;
        self.data[index + 1] += 1;
        self.data[index + 2] += 1;
    }

    pub fn invert_colors(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                for i in 0..3 {
                    self.data[index + i] = 255 - self.data[index + i];
                }
            }
        }
    }

    pub fn as_u8(&self) -> Vec<u8> {
        let mut data = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                for i in 0..4 {
                    data.push(self.data[index + i] as u8);
                }
            }
        }

        data
    }
}
