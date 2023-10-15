use crate::ppm::{Pixels, Ppm};
use glam::UVec3;

mod ppm;

const N: f32 = 255.999;

fn main() {
    let height = 256;
    let width = 256;

    let mut pixels: Pixels = Vec::with_capacity(height);

    for y in 0..height {
        eprintln!("Scan-lines remaining: {}", height - y);

        let mut row = Vec::new();
        for x in 0..width {
            let r = (x as f32) / (width as f32 - 1.) * N;
            let g = (y as f32) / (width as f32 - 1.) * N;
            let b = 0.;
            let v = UVec3::new(r as u32, g as u32, b as u32);
            row.push(v);
        }
        pixels.push(row);
    }

    let ppm = Ppm::new(width, height, pixels);
    println!("{ppm}");
    eprintln!("Done.");
}
