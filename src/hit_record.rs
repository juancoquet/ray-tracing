use crate::{point::Point, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}
