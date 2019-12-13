#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z}
    }

    pub const fn uniform(u: f32) -> Vec3 {
        Vec3::new(u,u,u)
    }

    pub const fn ones() -> Vec3 {
        Vec3::uniform(1.0)
    }

    pub const fn zero() -> Vec3 {
        Vec3::uniform(0.0)
    }

    pub const fn i_hat() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub const fn j_hat() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub const fn k_hat() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Vec3 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl crate::lerp::Lerp for Vec3 {
    fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Vec3 {
            x: self.x.lerp(&rhs.x, t),
            y: self.y.lerp(&rhs.y, t),
            z: self.z.lerp(&rhs.z, t),
        }
    }
}
// A space R2
// ntuple
// basic-vector
// struct R1;
// struct R2;
//Point<T,R1>
//Point<T,R2>
//Vector<T,D1>


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul_f32() {
        let expected = Vec3::new(0.0, 0.0, 0.0);
        let result = Vec3::ones() * 0.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn sub_vec3() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0),
            Vec3::new(100.0, 50.0, 25.0) - Vec3::new(99.0, 50.0, 26.0)
        );
    }

    #[test]
    fn i_hat() {
        assert_eq!(
            Vec3::i_hat(),
            Vec3::new(1.0, 0.0, 0.0)
        );
    }
    #[test]
    fn j_hat() {
        assert_eq!(
            Vec3::j_hat(),
            Vec3::new(0.0, 1.0, 0.0)
        );
    }
    #[test]
    fn k_hat() {
        assert_eq!(
            Vec3::k_hat(),
            Vec3::new(0.0, 0.0, 1.0)
        );
    }
    #[test]
    fn lerp() {
        use crate::lerp::Lerp;
        let a = Vec3::new(-1.0, -2.0, -3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), a.lerp(&b, 0.0));
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), a.lerp(&b, 0.5));
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), a.lerp(&b, 1.0));
    }
}
