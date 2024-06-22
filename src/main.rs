mod color;
mod hit_record;
mod hittable;
mod point;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use hit_record::HitRecord;
use hittable::Hittable;
use point::Point;
use ray::Ray;
use sphere::Sphere;
use std::cmp::max;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let provisional_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = max(provisional_height, 1);

    // world
    let sph1 = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    let hittables: [&dyn Hittable; 2] = [&sph1, &sph2];

    // camera
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
            let pixel_color = ray.color(&hittables);
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

fn hit_any(
    hittables: &[&dyn Hittable],
    ray: &Ray,
    ray_t_min: f64,
    ray_t_max: f64,
) -> Option<HitRecord> {
    let mut curr_closest = ray_t_max;
    let mut hit_record: Option<HitRecord> = None;

    for h in hittables {
        let hit_option = h.hit(ray, ray_t_min, curr_closest);
        if let Some(hit) = hit_option {
            curr_closest = hit.t;
            hit_record = Some(hit)
        }
    }
    hit_record
}

fn deg_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
