use cgmath::Array;
use cgmath::Vector3;

pub struct Obj {
    pub center_position: Vector3<f32>,
    pub triangles: Vec<Vector3<f32>>,
}

impl Obj {
    pub fn new(file_path: &str, center_position: Vector3<f32>) -> Self {
        let (models, materials) = 
            tobj::load_obj(
                file_path,
                &tobj::LoadOptions::default(),
            )
            .expect("Failed to obj load file");

        //let materials = materials.expect("Failed to load MTL file");

        println!("Number of models          = {}", models.len());
        //println!("Number of materials       = {}", materials.len());
    
        Self { center_position, triangles: vec![Vector3::from_value(0.0)] }
    }
}