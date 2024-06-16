use crate::{point::Point, vec3::Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    // Given a scalar value `t`, returns the point along the ray reached by traversing its direction
    // in proportion to the scalar
    pub fn at(&self, t: f64) -> Point {
        let scale = &self.direction * t;
        &self.origin + &scale
    }
}
