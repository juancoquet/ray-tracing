use crate::{color::Color, hit_record::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(incident: &Ray, hit_record: HitRecord, attenuation: &Color) -> Option<Ray>;
}
