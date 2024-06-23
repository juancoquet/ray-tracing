use crate::{
    color::Color, hit_record::HitRecord, hittable::Hittable, interval::Interval, point::Point,
    vec3::Vec3,
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    // Given a scalar value `t`, returns the point along the ray reached by traversing its direction
    // in proportion to the scalar.
    // P(t) = A + tB
    pub fn at(&self, t: f64) -> Point {
        let scale = &self.direction * t;
        &self.origin + &scale
    }

    pub fn color(&self, hittables: &[&dyn Hittable]) -> Color {
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);

        let hit_option = hit_any(hittables, self, Interval::new(0.0, std::f64::INFINITY));
        if let Some(hit) = hit_option {
            let shade = &hit.normal + &white;
            return shade * 0.5;
        }

        let direction_unit = self.direction.unit();
        // `a` is a value in the range [0,1] based on the direction_unit's y component
        let a = 0.5 * (direction_unit.y() + 1.0);
        lerp(a, &white, &blue)
    }
}

/// Linear blend/linear interpolation/lerp
/// Takes some value `a` in the range `[0,1]` and start & end values. Calculates the value that
/// pertains to `a` when placed in the gradient between start & end.
fn lerp(a: f64, start_value: &Color, end_value: &Color) -> Color {
    start_value * (1.0 - a) + end_value * a
}

fn hit_any(hittables: &[&dyn Hittable], ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let mut curr_closest = ray_t.max;
    let mut hit_record: Option<HitRecord> = None;

    for h in hittables {
        let hit_option = h.hit(ray, Interval::new(ray_t.min, curr_closest));
        if let Some(hit) = hit_option {
            curr_closest = hit.t;
            hit_record = Some(hit)
        }
    }
    hit_record
}
