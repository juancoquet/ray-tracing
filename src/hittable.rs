use crate::{hit_record::HitRecord, interval::Interval, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
