use std::fs::File;
use std::io::Write;

fn main() {
    let width = 256;
    let height = 256;

    let mut out = String::new();
    out.push_str(format!("P3\n{} {}\n255\n", width, height).as_str());

    for j in 0..height {
        for i in 0..width {
            let r = i as f32 / (height as f32 - 1.0);
            let g = j as f32 / (height as f32 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel = format!("{} {} {}\n", ir, ig, ib);
            out.push_str(pixel.as_str())
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
