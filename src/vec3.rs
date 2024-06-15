#[derive(Debug)]
pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { v: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }
    pub fn y(&self) -> f32 {
        self.v[1]
    }
    pub fn z(&self) -> f32 {
        self.v[2]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.v[0] == other.v[0] && self.v[1] == other.v[1] && self.v[2] == other.v[2]
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { v: [0.0, 0.0, 0.0] }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.v[0] += other.v[0];
        self.v[1] += other.v[1];
        self.v[2] += other.v[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.v[0] *= other;
        self.v[1] *= other;
        self.v[2] *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let exp = Vec3 { v: [x, y, z] };
        let res = Vec3::new(x, y, z);

        assert_eq!(exp, res);
    }

    #[test]
    fn test_xyz() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v = Vec3 { v: [x, y, z] };
        let (vx, vy, vz) = (v.x(), v.y(), v.z());

        assert_eq!(x, vx);
        assert_eq!(y, vy);
        assert_eq!(z, vz);
    }

    #[test]
    fn test_sub() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [a - x, b - y, c - z],
        };
        let res = v1 - v2;

        assert_eq!(exp, res);
    }

    #[test]
    fn test_add_assign() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let mut v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [a + x, b + y, c + z],
        };
        v1 += v2;

        assert_eq!(exp, v1);
    }

    #[test]
    fn test_mul_assign() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let x = 2.0;
        let mut v = Vec3 { v: [a, b, c] };
        let exp = Vec3 {
            v: [a * x, b * x, c * x],
        };
        v *= x;

        assert_eq!(exp, v);
    }
}
