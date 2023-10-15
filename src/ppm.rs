use glam::UVec3;
use std::fmt::{Display, Formatter};

const EMPTY: &Vec<UVec3> = &Vec::new();

pub type Pixels = Vec<Vec<UVec3>>;

pub struct Ppm {
    width: usize,
    height: usize,
    pixels: Pixels,
}

impl Ppm {
    pub fn new(width: usize, height: usize, pixels: Pixels) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl Display for Ppm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::with_capacity(self.height);
        lines.push(format!("P3\n{} {}\n255", self.width, self.height));

        for height in 0..self.height {
            for width in 0..self.width {
                let pixel = self
                    .pixels
                    .get(height)
                    .unwrap_or(EMPTY)
                    .get(width)
                    .unwrap_or(&UVec3::ZERO);

                lines.push(format!("{} {} {}", pixel.x, pixel.y, pixel.z));
            }
        }

        f.write_str(&lines.join("\n"))
    }
}
