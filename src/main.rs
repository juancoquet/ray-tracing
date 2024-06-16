mod color;
mod point;
mod vec3;

use color::{write_color, Color};
use point::Point;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

fn main() {
    let width = 256;
    let height = 256;

    let mut out = String::new();
    out.push_str(format!("P3\n{} {}\n255\n", width, height).as_str());

    for j in 0..height {
        print!("\r{}", progress_bar(j, height));
        for i in 0..width {
            let pixel = Color::new(
                i as f64 / (height as f64 - 1.0),
                j as f64 / (height as f64 - 1.0),
                0.0,
            );
            out.push_str(write_color(&pixel).as_str());
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

fn progress_bar(curr: i32, of: i32) -> String {
    let width = 80;
    let percent = curr as f64 / of as f64;
    let fill = (percent * width as f64) as i32;
    let empty = width - fill;
    format!(
        "[{}{}] {}%",
        "=".repeat(fill as usize),
        " ".repeat(empty as usize),
        (percent * 100.0) as i32
    )
}
