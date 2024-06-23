use std::{cmp::max, fs::File};

use crate::{color::Color, hittable::Hittable, point::Point, ray::Ray, sphere::Sphere, vec3::Vec3};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    centre: Point,
    top_left_pixel_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, hittables: &[&dyn Hittable]) -> String {
        let mut ppm = String::new();
        ppm.push_str(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_str());

        for y in 0..self.image_height {
            print!("\r{}", progress_bar(y, self.image_height));
            for x in 0..self.image_width {
                let pixel_centre = &self.top_left_pixel_loc
                    + &(&self.pixel_delta_u * x as f64)
                    + (&self.pixel_delta_v * y as f64);
                let ray_direction = &pixel_centre - &self.centre;
                let origin = Point::new(self.centre.x(), self.centre.y(), self.centre.z());
                let ray = Ray::new(origin, ray_direction);
                let pixel_color = ray.color(&hittables);
                ppm.push_str(write_color(&pixel_color).as_str());
            }
        }

        ppm
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let provisional_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = max(provisional_height, 1);

        // camera
        let focal_length = 1.0; // distance between the camera centre and the centre of the viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let centre = Point::new(0.0, 0.0, 0.0);

        // vectors across the horizontal and vertical space of the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // horizontal and vertical delta vectors between pixels; the distance between pixels in the
        // viewport
        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        // location of the top-left pixel. piexels should be inset by 1/2 * delta_v/delta_u
        let viewport_centre = &centre - &Vec3::new(0.0, 0.0, focal_length);
        let viewport_top_left = viewport_centre - viewport_u / 2.0 - viewport_v / 2.0;
        let top_left_pixel_loc = viewport_top_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

        Camera {
            image_width,
            image_height,
            centre,
            top_left_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
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

pub fn write_color(pixel_color: &Color) -> String {
    // translate the pixel's color value from the range [0, 1] -> [0, 255]
    let rbyte = (255.999 * pixel_color.x()) as i32;
    let gbyte = (255.999 * pixel_color.y()) as i32;
    let bbyte = (255.999 * pixel_color.z()) as i32;
    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}
