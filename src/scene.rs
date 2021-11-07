use crate::camera::Camera;
use crate::obj::Obj;
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