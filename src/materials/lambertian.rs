use crate::{color::Color, hit_record::HitRecord, ray::Ray, vec3::Vec3};

use super::material::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, incident: &Ray, hit: &HitRecord, attenuation: &Color) -> Ray {
        let mut scatter_direction = hit.normal + Vec3::random_unit();
        if scatter_direction.is_near_zero() {
            // 0-scatter direction leads to NaNs and unwanted infinities
            scatter_direction = hit.normal;
        }
        let scattered_ray = Ray::new(hit.point, scatter_direction);
        scattered_ray
    }
}
