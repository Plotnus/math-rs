use crate::vec3::Vec3;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Point {
        Point {x, y, z}
    }

    pub const fn uniform(u: f32) -> Point {
        Point::new(u,u,u)
    }

    pub const fn zero() -> Point {
        Point::uniform(0.0)
    }

    pub const fn ones() -> Point {
        Point::uniform(1.0)
    }
}

impl Add<Vec3> for Point {
    type Output = Point;
    fn add(self, v: Vec3) -> Point {
        Point {
            x: self.x + v.x,
            y: self.x + v.y,
            z: self.z + v.z,
        }
    }
}

impl AddAssign<Vec3> for Point {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Vec3> for Point {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for Point {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for Point {
    type Output = Self;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vec3> for Point {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<&Vec3> for Point {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vec3::Vec3;

    #[test]
    fn zero() {
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn ones() {
        assert_eq!(Vec3::ones(), Vec3::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn uniform() {
        assert_eq!(Vec3::uniform(42.0), Vec3::new(42.0, 42.0, 42.0));
    }
    #[test]
    fn add_vec3d() {
        assert_eq!(Point::new(2.0, 2.0, 2.0), Point::ones() + Vec3::ones());
    }
}
