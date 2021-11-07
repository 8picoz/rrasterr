use crate::camera;
use crate::camera::Camera;
use crate::obj;
use crate::obj::Obj;
use crate::screen;
use crate::screen::Screen;

pub struct Scene {
    camera: Camera,
    obj: Obj,
}

impl Scene {
    pub fn new(camera: Camera, obj: Obj) -> Self {
        Self { camera, obj }
    }
}