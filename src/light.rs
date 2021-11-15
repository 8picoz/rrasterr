use cgmath::InnerSpace;

use crate::Vec3f;

pub struct Light {
    pub direction: Vec3f,
}

impl Light {
    pub fn new(direction: Vec3f) -> Self {
        Light { direction: direction.normalize() }
    }
}