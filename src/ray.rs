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

    pub fn color(&self, hittables: &[&dyn Hittable], max_reflections: i32) -> Color {
        if max_reflections <= 0 {
            // Once we've hit the maximum number of reflections, contribute no more light to the
            // scene
            return Color::new(0.0, 0.0, 0.0);
        }
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);

        let hit_option = hit_any(hittables, self, Interval::new(0.001, std::f64::INFINITY));
        if let Some(hit) = hit_option {
            let reflection_direction = Vec3::random_on_hemisphere(&hit.normal);
            let reflection_ray = Ray::new(hit.point, reflection_direction);
            let reflection_color = reflection_ray.color(hittables, max_reflections - 1);
            return reflection_color * 0.5;
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
