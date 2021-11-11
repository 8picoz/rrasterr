use std::ops::Index;

use cgmath::Vector4;

type Vec4f = Vector4<f32>;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec4f,
    pub y: Vec4f,
    pub z: Vec4f,
}

pub struct TriangleIter<'a> {
    triangle: &'a Triangle,
    now: usize,
}

impl Triangle {
    pub fn new(x: Vec4f, y: Vec4f, z: Vec4f) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> TriangleIter{
        TriangleIter { triangle: self, now: 0 }
    }
}

impl<'a> Iterator for TriangleIter<'a> {
    type Item = Vec4f;
    fn next(&mut self) -> Option<Self::Item> {
        if self.now == 0 {
            Some(self.triangle.x)
        } else if self.now == 1 {
            Some(self.triangle.y)
        } else if self.now == 2 {
            Some(self.triangle.z)
        } else {
            None
        }
    }
}

impl Index<usize> for Triangle {
    type Output = Vec4f;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx == 0 {
            &self.x
        } else if idx == 1 {
            &self.y
        } else if idx == 2 {
            &self.z
        } else {
            panic!("Index out of bounds");
        }
    }
}