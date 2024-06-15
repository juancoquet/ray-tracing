mod vec3;
mod point;

use std::fs::File;
use std::io::Write;
use vec3::Vec3;
use point::Point;


fn main() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let point = Point::new(4.0, 5.0, 6.0);

    let width = 256;
    let height = 256;

    let mut out = String::new();
    out.push_str(format!("P3\n{} {}\n255\n", width, height).as_str());

    for j in 0..height {
        print!("\r{}", progress_bar(j, height));
        for i in 0..width {
            let r = i as f32 / (height as f32 - 1.0);
            let g = j as f32 / (height as f32 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel = format!("{} {} {}\n", ir, ig, ib);
            out.push_str(pixel.as_str());
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

fn progress_bar(curr: i32, of: i32) -> String {
    let width = 80;
    let percent = curr as f32 / of as f32;
    let fill = (percent * width as f32) as i32;
    let empty = width - fill;
    format!(
        "[{}{}] {}%",
        "=".repeat(fill as usize),
        " ".repeat(empty as usize),
        (percent * 100.0) as i32
    )
}
