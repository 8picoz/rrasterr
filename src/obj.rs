use std::borrow::Cow;

use cgmath::InnerSpace;
use cgmath::Matrix4;

use crate::Vec3f;
use crate::Vec4f;
use crate::triangle::Triangle;
use crate::vertex::Vertex;

pub struct Obj {
    pub center_position: Vec3f,
    pub triangles: Vec<Triangle>,
}

impl Obj {
    pub fn new<'a>(file_path: impl Into<Cow<'a, str>>, center_position: Vec3f) -> Self {
        let file_path: &str = &file_path.into();
        
        let (models, _) = 
            tobj::load_obj(
                file_path,
                &tobj::LoadOptions { triangulate: true, ..tobj::LoadOptions::default()}
            )
            .expect("Failed to obj load file");

        //println!("Number of models          = {}", models.len());
    
        let mut triangles = vec![];
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            
            let face_count = mesh.indices.len() / 3;

            //println!("model[{}].face_count       = {}", _i, face_count);
            //println!("position[{}].positions     = {}", _i, mesh.positions.len() / 3);

            let mut next_face = 0;
            for _face in 0..face_count {
                let end = next_face + 3;

                let face_indices = &mesh.indices[next_face..end];
                //println!("face[{}].indices          = {:?}", _face, face_indices);

                let p0 = Vec3f::new(mesh.positions[(3 * face_indices[0]) as usize], mesh.positions[(3 * face_indices[0] + 1) as usize], mesh.positions[(3 * face_indices[0] + 2) as usize]);
                let p1 = Vec3f::new(mesh.positions[(3 * face_indices[1]) as usize], mesh.positions[(3 * face_indices[1] + 1) as usize], mesh.positions[(3 * face_indices[1] + 2) as usize]);
                let p2 = Vec3f::new(mesh.positions[(3 * face_indices[2]) as usize], mesh.positions[(3 * face_indices[2] + 1) as usize], mesh.positions[(3 * face_indices[2] + 2) as usize]);
                
                //法線を自前で計算
                //CCW
                let n0 = (p1 - p0).cross(p2 - p0).normalize();
                let n1 = (p2 - p1).cross(p0 - p1).normalize();
                let n2 = (p0 - p2).cross(p1 - p2).normalize();

                let v0 = Vertex::new(Vec4f::new(p0.x, p0.y, p0.z, 1.0), n0);
                let v1 = Vertex::new(Vec4f::new(p1.x, p1.y, p1.z, 1.0), n1);
                let v2 = Vertex::new(Vec4f::new(p2.x, p2.y, p2.z, 1.0), n2);

                triangles.push(Triangle::new(v0, v1, v2));
            
                next_face = end;
            }
        }

        Self { center_position, triangles }
    }

    pub fn convert(&mut self, mat: Matrix4<f32>) {
        for tri in &mut self.triangles {
            tri.x.convert(mat);
            tri.y.convert(mat);
            tri.z.convert(mat);
        }
    }
}