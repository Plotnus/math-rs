use crate::vector::Vec3d;

struct _Point2d {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}
impl Point3d {
    pub const ZERO: Point3d = Point3d { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONES: Point3d = Point3d { x: 1.0, y: 1.0, z: 1.0 };
    pub fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d {x, y, z}
    }
}
impl core::ops::Add<Vec3d> for Point3d {
    fn add(self, v: Vec3d) -> Point3d {
        Point3d {
            x: self.x + v.x,
            y: self.x + v.y,
            z: self.z + v.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point3d;
    use crate::vector::Vec3d;

    #[test]
    fn point3d_add_vec3d() {
        assert_eq!(Point3d::new(2.0, 2.0, 2.0), Point3d::ONES + Vec3d::ONES);
    }
}
