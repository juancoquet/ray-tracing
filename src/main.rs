mod color;
mod point;
mod ray;
mod vec3;

use color::write_color;
use point::Point;
use ray::Ray;
use std::cmp::max;
use std::fs::File;
use std::io::Write;
use vec3::{dot, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let provisional_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = max(provisional_height, 1);

    let focal_length = 1.0; // distance between the camera centre and the centre of the viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_centre = Point::new(0.0, 0.0, 0.0);

    // vectors across the horizontal and vertical space of the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // horizontal and vertical delta vectors between pixels; the distance between pixels in the
    // viewport
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // location of the top-left pixel. piexels should be inset by 1/2 * delta_v/delta_u
    let viewport_centre = &camera_centre - &Vec3::new(0.0, 0.0, focal_length);
    let viewport_top_left = viewport_centre - viewport_u / 2.0 - viewport_v / 2.0;
    let top_left_pixel_loc = viewport_top_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    let mut out = String::new();
    out.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    for y in 0..image_height {
        print!("\r{}", progress_bar(y, image_height));
        for x in 0..image_width {
            let pixel_centre =
                &top_left_pixel_loc + &(&pixel_delta_u * x as f64) + (&pixel_delta_v * y as f64);
            let ray_direction = &pixel_centre - &camera_centre;
            let origin = Point::new(camera_centre.x(), camera_centre.y(), camera_centre.z());
            let ray = Ray::new(origin, ray_direction);
            let pixel_color = ray.color();
            out.push_str(write_color(&pixel_color).as_str());
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

/// The equation for a sphere centered at point `(Cx, Cy, Cz)` with radius `r` is:
/// `(Cx - Px)^2 + (Cy - Py)^2 + (Cz - Pz)^2 = r^2`
/// where `P` is a point on the sphere and `C` is the center of the sphere.
/// That is to say, any point `P` that satisfies the above equation *is on the surface of the sphere*.
/// Given that the vector from `P` to `C` is `(C - P)`, i.e., `(Cx - Px, Cy - Py, Cz - Pz)`:
/// `(C - P) = (Cx - Px, Cy - Py, Cz - Pz)`
/// (Note that the magnitude of `(C - P)` is `r`)
/// Then the dot product of `(C - P)` with itself is:
/// `(C - P) ⋅ (C - P) = (Cx - Px, Cy - Py, Cz - Pz) ⋅ (Cx - Px, Cy - Py, Cz - Pz)`
/// `(C - P) ⋅ (C - P) = (Cx - Px)^2 + (Cy - Py)^2 + (Cz - Pz)^2`
/// Note that the right-hand side is the same as the definition of `r^2`, so:
/// `(C - P) ⋅ (C - P) = r^2`
/// Thus, any point `P` which satisfies the above equation is on the surface of the sphere.
///
/// ---
/// Given some ray and some sphere, we'd like to know if any point along that ray is on the surface
/// of the sphere.
fn hit_sphere(sphere_centre: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = sphere_centre - &ray.origin;
    let a = ray.direction.len_sq();
    let h = dot(&ray.direction, &oc);
    let c = oc.len_sq() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
