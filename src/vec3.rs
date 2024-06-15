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
}
