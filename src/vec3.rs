use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign}; 
#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// block for associated fn
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

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
        a * (1.0 - t) + b * t
    }

    // decided to have the default lerp as unclamed since we do
    // additional operations for the `_clamped` version
    // this follows the natural progression of building things from
    // their pieces
    pub fn lerp_clamped(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
        Vec3::lerp(a, b, t.clamp(0.0, 1.0))
    }

    // TODO: implement
    // slerp_clamped
    // slerp
    // clamped_magnitude
    // normalized

    // We do not implement directions here since that's a higher
    // level decision. Eg. it can vary from engine to software.
    // It would be nice to make it easy to transform from one to
    // another tho
    // up, down, back, forward, left, right
    // equals
    // normalized_checked
    // normalized_fast
}

impl Vec3 {
    fn mag_squared(&self) -> f32 {
        Vec3::dot(&self, &self)
    }

    fn mag(&self) -> f32 {
        self.mag_squared().sqrt()
    }

    fn len_sq(&self) -> f32 {
        Vec3::dot(&self, &self)
    }

    fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}


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
    fn sub() {
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
        let a = Vec3::new(-1.0, -2.0, -3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), Vec3::lerp(&a, &b, 0.0));
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), Vec3::lerp(&a, &b, 0.5));
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::lerp(&a, &b, 1.0));
    }

    #[test]
    fn neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), -v);
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), -Vec3::new(0.0,0.0,0.0));
    }

}
