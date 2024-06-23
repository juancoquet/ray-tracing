use crate::{
    hit_record::HitRecord, hittable::Hittable, interval::Interval, material::Material,
    point::Point, ray::Ray, vec3::dot,
};

pub struct Sphere<'a> {
    centre: Point,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(centre: Point, radius: f64, material: &'a dyn Material) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    /**
    The equation for a sphere centered at point `C = (Cx, Cy, Cz)` with radius `r` is:
    `(Cx - Px)^2 + (Cy - Py)^2 + (Cz - Pz)^2 = r^2`
    where `P` is a point `(Px, Py, Pz)` on the sphere and `C` is the center of the sphere.
    That is to say, any point `P` that satisfies the above equation *is on the surface of the sphere*.
    Given that the vector from `P` to `C` is `(C - P)`, i.e., `(Cx - Px, Cy - Py, Cz - Pz)`:
    `(C - P) = (Cx - Px, Cy - Py, Cz - Pz)`
    (Note that the magnitude of `(C - P)` is `r`)
    Then the dot product of `(C - P)` with itself is:
    `(C - P) ⋅ (C - P) = (Cx - Px, Cy - Py, Cz - Pz) ⋅ (Cx - Px, Cy - Py, Cz - Pz)`
    `(C - P) ⋅ (C - P) = (Cx - Px)^2 + (Cy - Py)^2 + (Cz - Pz)^2`
    Note that the right-hand side is the same as the definition of `r^2`, so:
    `(C - P) ⋅ (C - P) = r^2`
    Thus, any point `P` which satisfies the above equation is on the surface of the sphere.

    ---
    Given some ray and some sphere, we'd like to know if any point along that ray is on the surface
    of the sphere.
    */
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = &self.centre - &ray.origin;
        let a = ray.direction.len_sq();
        let h = dot(&ray.direction, &oc);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // find the nearest root that lies within the acceptable t range
        let root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            return None;
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal_unit = (&point - &self.centre) / self.radius;
        let front_face = dot(&ray.direction, &outward_normal_unit) < 0.0;
        let normal = if front_face {
            outward_normal_unit
        } else {
            -outward_normal_unit
        };

        Some(HitRecord::new(point, normal, t, front_face, self.material))
    }
}
