use crate::{Vec3f, Vec4f};

struct Vertex {
    pub point: Vec4f,
    pub normal: Vec3f,
}

impl Vertex {
    pub fn new(point: Vec4f, normal: Vec3f) -> Self {
        Self { point, normal }
    }
}