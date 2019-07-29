use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Image {
    data: Vec<u64>,
    width: u64,
    height: u64,
}

#[wasm_bindgen]
impl Image {
    pub fn init(width: u64, height: u64) -> Image {
        let mut image_data = Image {
            data: vec![],
            width: width,
            height: height,
        };

        image_data.init_image();

        image_data
    }

    pub fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                for i in 0..3 {
                    self.data[index + i] = 0;
                }

                self.data[index + 3] = 255;
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

    pub fn gamma_correction(&mut self, gamma: f64) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width + x) * 4) as usize;

                for i in 0..3 {
                    self.data[index + i] =
                        ((self.data[index + i] as f64 / 255.0).powf(1.0 / gamma) * 255.0) as u64;
                }
            }
        }
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

impl Image {
    pub fn put_pixel(&mut self, x: u64, y: u64, color: [u8; 3]) {
        if x > self.width - 1 {
            return;
        }

        if y > self.height - 1 {
            return;
        }

        let index = ((y * self.width as u64 + x) * 4) as usize;

        for i in 0..3 {
            self.data[index + i] += color[i] as u64;
        }
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
}
