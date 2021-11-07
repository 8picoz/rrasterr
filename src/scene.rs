use cgmath::Vector4;

use crate::camera::Camera;
use crate::obj::Obj;
use crate::triangle::Triangle;
use crate::world_converter::projection_matrix;
use crate::world_converter::view_matrix;

pub struct Scene {
    camera: Camera,
    obj: Obj,
}

impl Scene {
    pub fn new(camera: Camera, obj: Obj) -> Self {
        Self { camera, obj }
    }

    pub fn view_convert(&mut self) {
        let view_m = view_matrix(&self.camera);
        self.obj.convert(view_m);
    }

    pub fn projection_convert(&mut self) {
        let proj_m = projection_matrix(&self.camera.screen);
        self.obj.convert(proj_m);
    }

    pub fn perspective_division(&mut self) {
        self.obj.triangles = self.obj.triangles.iter().map(
            |tri| 
                Triangle::new(
                    tri.x / tri.x.w,
                    tri.y / tri.y.w,
                    tri.z / tri.z.w
                )
        ).collect();
    }
}

impl AsRef<Scene> for Scene {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Scene> for Scene {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}