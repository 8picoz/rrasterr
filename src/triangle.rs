use cgmath::Vector3;

type Vec3f = Vector3<f32>;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
}

impl Triangle {
    pub fn new(x: Vec3f, y: Vec3f, z: Vec3f) -> Self {
        Self { x, y, z }
    }
}