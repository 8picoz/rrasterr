use cgmath::InnerSpace;
use cgmath::Vector3;

type Vec3f = Vector3<f32>;

pub struct Light {
    pub direction: Vec3f,
}

impl Light {
    pub fn new(direction: Vec3f) -> Self {
        Light { direction: direction.normalize() }
    }
}