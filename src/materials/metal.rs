use crate::{color::Color, hit_record::HitRecord, point::Point, ray::Ray};

use super::material::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, incident: &Ray, hit: &HitRecord) -> (Ray, Color) {
        let reflection_direction = incident.direction.reflect(&hit.normal);
        let reflection_origin = Point::new(hit.point.x(), hit.point.y(), hit.point.z());
        let reflected_ray = Ray::new(reflection_origin, reflection_direction);
        let color = Color::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        (reflected_ray, color)
    }
}
