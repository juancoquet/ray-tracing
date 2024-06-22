use crate::{point::Point, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    point: Point,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vec3, t: f64) -> Self {
        HitRecord { point, normal, t }
    }
}
