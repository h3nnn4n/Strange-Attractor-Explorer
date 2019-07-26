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

    pub fn normalize_image(&self) -> Vec<u8> {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut normalized_data = Vec::new();

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

                normalized_data.push(((self.data[index + 0] as f64 / r as f64) * 255 as f64) as u8);
                normalized_data.push(((self.data[index + 1] as f64 / g as f64) * 255 as f64) as u8);
                normalized_data.push(((self.data[index + 2] as f64 / b as f64) * 255 as f64) as u8);
                normalized_data.push(255);
            }
        }

        normalized_data
    }

    pub fn put_pixel(&mut self, (x, y): (u64, u64)) {
        let index = ((y * self.width as u64 + x) * 4) as usize;

        self.data[index + 0] += 1;
        self.data[index + 1] += 1;
        self.data[index + 2] += 1;
    }
}
