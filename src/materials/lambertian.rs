use crate::{
    color::Color,
    hit_record::HitRecord,
    point::Point,
    ray::Ray,
    vec3::{dot, Vec3},
};

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
    fn scatter(&self, incident: &Ray, hit: &HitRecord) -> (Ray, Color) {
        let reflect_direction = reflect(&incident.direction, &hit.normal);
        let scatter_origin = Point::new(hit.point.x(), hit.point.y(), hit.point.z());
        let scattered_ray = Ray::new(scatter_origin, reflect_direction);
        let color = Color::new(self.albedo.x(), self.albedo.y(), self.albedo.z());
        (scattered_ray, color)
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    let projection = normal * dot(&incident, &normal);
    let bb = projection * 2.0;
    incident - &bb
}
