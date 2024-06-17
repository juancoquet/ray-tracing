use crate::{color::Color, hit_sphere, point::Point, vec3::Vec3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
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

    pub fn color(&self) -> Color {
        let sphere_centre = Point::new(0.0, 0.0, -1.0);
        if hit_sphere(&sphere_centre, 0.5, &self) {
            return Color::new(1.0, 0.0, 0.0);
        }
        let direction_unit = self.direction.unit();
        // `a` is a value in the range [0,1] based on the direction_unit's y component
        let a = 0.5 * (direction_unit.y() + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);
        lerp(a, &white, &blue)
    }
}

/// Linear blend/linear interpolation/lerp
/// Takes some value `a` in the range [0,1] and start & end values. Calculates the value that
/// pertains to `a` when placed in the gradient between start & end.
fn lerp(a: f64, start_value: &Color, end_value: &Color) -> Color {
    start_value * (1.0 - a) + end_value * a
}
