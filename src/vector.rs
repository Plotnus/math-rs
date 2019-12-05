struct _Vec2d {
    x: f32,
    y: f32,
}
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d {
    pub const ONES: Vec3d = Vec3d {x: 1.0, y: 1.0, z: 1.0 };
    pub const _ZERO: Vec3d = Vec3d {x: 0.0, y: 0.0, z: 0.0 };

    fn _new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d { x, y, z}
    }
}

