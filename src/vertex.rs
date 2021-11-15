use cgmath::Matrix4;

use crate::{Vec3f, Vec4f};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub point: Vec4f,
    pub normal: Vec3f,
}

impl Vertex {
    pub fn new(point: Vec4f, normal: Vec3f) -> Self {
        Self { point, normal }
    }

    pub fn convert(&mut self, mat: Matrix4<f32>) {
        self.point = mat * self.point;
    }
}