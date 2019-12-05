use crate::vector::Vec3;

struct _Point2d {
    x: f32,
    y: f32,
}
#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}
impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONES: Point = Point { x: 1.0, y: 1.0, z: 1.0 };
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

// implementing the core ops for Point
impl core::ops::Add<Vec3> for Point {
    type Output = Point;
    fn add(self, v: Vec3) -> Point {
        Point {
            x: self.x + v.x,
            y: self.x + v.y,
            z: self.z + v.z,
        }
    }
}
impl core::ops::Add<&Vec3> for Point {
    type Output = Point;
    fn add(self, v: &Vec3) -> Point {
        Point {
            x: self.x + v.x,
            y: self.x + v.y,
            z: self.z + v.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vec3;

    #[test]
    fn add_vec3d() {
        assert_eq!(Point::new(2.0, 2.0, 2.0), Point::ones() + Vec3::ones());
    }
    #[test]
    fn add_ref_vec3d() {
        let v = Vec3::new(1.0,1.0,1.0);
        assert_eq!(Point::new(2.0, 2.0, 2.0), Point::ones() + &v);

        println!("{:?}", v);
    }
}
