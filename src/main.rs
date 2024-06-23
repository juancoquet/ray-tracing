mod camera;
mod color;
mod hit_record;
mod hittable;
mod interval;
mod materials;
mod point;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use point::Point;
use sphere::Sphere;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut camera = Camera::default();

    let sph1 = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sph2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    let hittables: [&dyn Hittable; 2] = [&sph1, &sph2];

    let reflection_depth = 50;
    let ppm = camera.render(&hittables, reflection_depth);

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}

fn deg_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
