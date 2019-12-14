use crate::vec3::{Vec3};
use crate::point::{Point};

#[repr(align(16))]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    fn from_vec3(v: &Vec3) -> Vec4 {
        Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }

    fn from_point(p: &Point) -> Vec4 {
        Vec4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::vec4::Vec4;
    #[test]
    fn alignment() {
        assert_eq!(std::mem::size_of::<Vec4>(), std::mem::align_of::<Vec4>());
    }
}
