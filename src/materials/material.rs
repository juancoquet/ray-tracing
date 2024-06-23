use crate::{color::Color, hit_record::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, incident: &Ray, hit: &HitRecord) -> (Ray, Color);
}
