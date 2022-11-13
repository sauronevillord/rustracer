use std::fs::File;
use std::io::{prelude::*, Result};

use trace_math::vec3::{RGBColor};

pub fn initialize_ppm(width: u32, height: u32) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}

pub fn add_pixel_to_ppm(img: &mut String, color: RGBColor, samples_per_pixels: u32) {
    let scale = 1. / samples_per_pixels as f64;
    let scaled_color = color * scale;
    let (r, g, b) = (scaled_color.x.sqrt(), scaled_color.y.sqrt(), scaled_color.z.sqrt());

    let ir = (256. * r.clamp(0., 0.999999)) as i32;
    let ig = (256. * g.clamp(0., 0.999999)) as i32;
    let ib = (256. * b.clamp(0., 0.999999)) as i32;

    *img = format!("{}\n{} {} {}\n", img, ir, ig, ib);
}

pub fn save_ppm(img: String) -> Result<()> {
    let mut file = File::create("out.ppm").unwrap();
    file.write_all(img.as_bytes()).unwrap();
    Ok(())
}
