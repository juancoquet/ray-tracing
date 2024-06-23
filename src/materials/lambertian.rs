use crate::{color::Color, hit_record::HitRecord, point::Point, ray::Ray, vec3::Vec3};

use super::material::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> (Ray, Color) {
        let mut scatter_direction = &hit.normal + &Vec3::random_unit();
        if scatter_direction.is_near_zero() {
            // 0-scatter direction leads to NaNs and unwanted infinities
            scatter_direction = Point::new(hit.normal.x(), hit.normal.y(), hit.normal.z());
        }
        let scatter_origin = Point::new(hit.point.x(), hit.point.y(), hit.point.z());
        let scattered_ray = Ray::new(scatter_origin, scatter_direction);
        let color = Color::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        (scattered_ray, color)
    }
}
