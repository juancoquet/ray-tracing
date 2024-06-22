use crate::{hit_record::HitRecord, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;
}
