use std::borrow::Cow;

use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Vector4;

use crate::triangle::Triangle;

pub struct Obj {
    pub center_position: Vector3<f32>,
    pub triangles: Vec<Triangle>,
}

impl Obj {
    pub fn new<'a>(file_path: impl Into<Cow<'a, str>>, center_position: Vector3<f32>) -> Self {
        let file_path: &str = &file_path.into();
        
        let (models, _) = 
            tobj::load_obj(
                file_path,
                &tobj::LoadOptions { triangulate: true, ..tobj::LoadOptions::default()}
            )
            .expect("Failed to obj load file");

        println!("Number of models          = {}", models.len());
    
        let mut triangles = vec![];
        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            
            let face_count = mesh.indices.len() / 3;

            println!("model[{}].face_count       = {}", i, face_count);
            println!("position[{}].positions     = {}", i, mesh.positions.len() / 3);

            let mut next_face = 0;
            for _face in 0..face_count {
                let end = next_face + 3;

                let face_indices = &mesh.indices[next_face..end];
                println!("face[{}].indices          = {:?}", _face, face_indices);

                triangles.push(Triangle::new(
                    Vector4::new(mesh.positions[(3 * face_indices[0]) as usize], mesh.positions[(3 * face_indices[0] + 1) as usize], mesh.positions[(3 * face_indices[0] + 2) as usize], 1.0),
                    Vector4::new(mesh.positions[(3 * face_indices[1]) as usize], mesh.positions[(3 * face_indices[1] + 1) as usize], mesh.positions[(3 * face_indices[1] + 2) as usize], 1.0),
                    Vector4::new(mesh.positions[(3 * face_indices[2]) as usize], mesh.positions[(3 * face_indices[2] + 1) as usize], mesh.positions[(3 * face_indices[2] + 2) as usize], 1.0),
                ));
            
                next_face = end;
            }
        }

        Self { center_position, triangles }
    }

    pub fn convert(&mut self, mat: Matrix4<f32>) {
        //immutable
        self.triangles = self.triangles.iter().map(|tri| Triangle::new(mat * tri.x, mat * tri.y, mat * tri.z)).collect();
    }
}