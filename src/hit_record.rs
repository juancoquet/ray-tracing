use crate::{materials::material::Material, point::Point, vec3::Vec3};

pub struct HitRecord<'a> {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Self {
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
