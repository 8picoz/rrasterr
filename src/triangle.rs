use std::ops::Index;

use crate::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vertex,
    pub y: Vertex,
    pub z: Vertex,
}

pub struct TriangleIter<'a> {
    triangle: &'a Triangle,
    now: usize,
}

impl Triangle {
    pub fn new(x: Vertex, y: Vertex, z: Vertex) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> TriangleIter{
        TriangleIter { triangle: self, now: 0 }
    }
}

impl<'a> Iterator for TriangleIter<'a> {
    type Item = Vertex;
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
    type Output = Vertex;

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