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

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }
    pub fn len_sq(&self) -> f32 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn unit(&self) -> Vec3 {
        let reciprocal = 1.0 / self.len();
        Vec3 {
            v: [
                self.v[0] * reciprocal,
                self.v[1] * reciprocal,
                self.v[2] * reciprocal,
            ],
        }
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

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.v[0] -= other.v[0];
        self.v[1] -= other.v[1];
        self.v[2] -= other.v[2];
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
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

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Vec3 {
            v: [self.v[0] * other, self.v[1] * other, self.v[2] * other],
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.v[0] *= other;
        self.v[1] *= other;
        self.v[2] *= other;
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2],
            ],
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        // use the reciprocal of the divisor to avoid division as division is more expensive than
        // multiplication
        let reciprocal = 1.0 / other;
        Vec3 {
            v: [
                self.v[0] * reciprocal,
                self.v[1] * reciprocal,
                self.v[2] * reciprocal,
            ],
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        // use the reciprocal of the divisor to avoid division as division is more expensive than
        // multiplication
        let reciprocal = 1.0 / other;
        self.v[0] *= reciprocal;
        self.v[1] *= reciprocal;
        self.v[2] *= reciprocal;
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.v[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.v[i]
    }
}

fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.v[0] * v2.v[0] + v1.v[1] * v2.v[1] + v1.v[2] * v2.v[2]
}

fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        v: [
            v1.v[1] * v2.v[2] - v1.v[2] * v2.v[1],
            v1.v[2] * v2.v[0] - v1.v[0] * v2.v[2],
            v1.v[0] * v2.v[1] - v1.v[1] * v2.v[0],
        ],
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
    fn test_sub_assign() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let mut v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [a - x, b - y, c - z],
        };
        v1 -= v2;

        assert_eq!(exp, v1);
    }

    #[test]
    fn test_add() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [a + x, b + y, c + z],
        };
        let v3 = v1 + v2;

        assert_eq!(exp, v3);
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
    fn test_mul_f32() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let x = 2.0;
        let v = Vec3 { v: [a, b, c] };
        let exp = Vec3 {
            v: [a * x, b * x, c * x],
        };
        let res = v * x;

        assert_eq!(exp, res);
    }

    #[test]
    fn test_mul_assign_f32() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let x = 2.0;
        let mut v = Vec3 { v: [a, b, c] };
        let exp = Vec3 {
            v: [a * x, b * x, c * x],
        };
        v *= x;

        assert_eq!(exp, v);
    }

    #[test]
    fn test_mul_vec3() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [a * x, b * y, c * z],
        };
        let res = v1 * v2;

        assert_eq!(exp, res);
    }

    #[test]
    fn test_div() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let x = 2.0;
        let v = Vec3 { v: [a, b, c] };
        let exp = Vec3 {
            v: [a / x, b / x, c / x],
        };
        let res = v / x;

        assert_eq!(exp, res);
    }

    #[test]
    fn test_div_assign() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let x = 2.0;
        let mut v = Vec3 { v: [a, b, c] };
        let exp = Vec3 {
            v: [a / x, b / x, c / x],
        };
        v /= x;

        assert_eq!(exp, v);
    }

    #[test]
    fn test_len_sq() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v = Vec3 { v: [x, y, z] };
        let exp = x * x + y * y + z * z;
        let res = v.len_sq();

        assert_eq!(exp, res);
    }

    #[test]
    fn test_index() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v = Vec3 { v: [x, y, z] };

        assert_eq!(x, v[0]);
        assert_eq!(y, v[1]);
        assert_eq!(z, v[2]);
    }

    #[test]
    fn test_index_mut() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let mut v = Vec3 { v: [x, y, z] };
        let exp = 4.0;
        v[0] = exp;

        assert_eq!(exp, v[0]);
    }

    #[test]
    fn test_dot() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = a * x + b * y + c * z;
        let res = dot(v1, v2);

        assert_eq!(exp, res);
    }

    #[test]
    fn test_cross() {
        let (a, b, c) = (4.0, 5.0, 6.0);
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v1 = Vec3 { v: [a, b, c] };
        let v2 = Vec3 { v: [x, y, z] };
        let exp = Vec3 {
            v: [b * z - c * y, c * x - a * z, a * y - b * x],
        };
        let res = cross(v1, v2);

        assert_eq!(exp, res);
    }

    #[test]
    fn test_unit() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v = Vec3 { v: [x, y, z] };
        let len = v.len();
        let exp = Vec3 {
            v: [x / len, y / len, z / len],
        };
        let res = v.unit();

        assert_eq!(exp, res);
    }
}
