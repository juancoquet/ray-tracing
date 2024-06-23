use crate::{
    color::Color, hit_record::HitRecord, interval::Interval, point::Point, ray::Ray, vec3::Vec3,
};

use super::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz_norm = Interval::new(0.0, 1.0).clamp(fuzz);
        Metal {
            albedo,
            fuzz: fuzz_norm,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, incident: &Ray, hit: &HitRecord) -> (Ray, Color) {
        let reflection_direction = incident.direction.reflect(&hit.normal);
        let reflection_direction_fuzzy =
            reflection_direction.unit() + (Vec3::random_unit() * self.fuzz);
        let reflection_origin = Point::new(hit.point.x(), hit.point.y(), hit.point.z());
        let reflected_ray = Ray::new(reflection_origin, reflection_direction_fuzzy);
        let color = Color::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        (reflected_ray, color)
    }
}
