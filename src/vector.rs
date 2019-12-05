struct _Vec2d {
    x: f32,
    y: f32,
}
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ONES: Vec3 = Vec3 {x: 1.0, y: 1.0, z: 1.0 };
    pub const _ZERO: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z}
    }
}

