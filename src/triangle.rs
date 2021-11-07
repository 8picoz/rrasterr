use cgmath::Vector4;

type Vec4f = Vector4<f32>;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec4f,
    pub y: Vec4f,
    pub z: Vec4f,
}

impl Triangle {
    pub fn new(x: Vec4f, y: Vec4f, z: Vec4f) -> Self {
        Self { x, y, z }
    }
}