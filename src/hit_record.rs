use crate::{point::Point, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    point: Point,
    normal: Vec3,
    t: f64,
    front_face: bool,
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
