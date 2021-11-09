use std::borrow::Cow;
use std::io::Result;

use cgmath::Array;
use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Vector4;

use crate::camera::Camera;
use crate::image::Image;
use crate::obj::Obj;
use crate::screen::Screen;
use crate::triangle::Triangle;
use crate::world_converter::projection_matrix;
use crate::world_converter::view_matrix;

pub struct Scene {
    image: Image,
    camera: Camera,
    obj: Obj,
}

impl Scene {
    pub fn new(image: Image, camera: Camera, obj: Obj) -> Self {
        Self { image, camera, obj }
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
                    //wはzとは逆方向に伸びたベクトル(?)
                    tri.x / tri.x.w,
                    tri.y / tri.y.w,
                    tri.z / tri.z.w
                )
        ).collect();
    }

    pub fn generate_image(&mut self, output_path: impl Into<Cow<'static , str>>) -> Result<()> {
        for tri in &self.obj.triangles {
            let pixel_0 = Scene::viewport_convert(tri.x, &self.camera.screen);
            let pixel_1 = Scene::viewport_convert(tri.y, &self.camera.screen);
            let pixel_2 = Scene::viewport_convert(tri.z, &self.camera.screen);

            let (w, h) = self.image.get_size();

            let pixel_0 = Vector2::new(pixel_0.x * w as f32, pixel_0.y * h as f32);
            let pixel_1 = Vector2::new(pixel_1.x * w as f32, pixel_1.y * h as f32);
            let pixel_2 = Vector2::new(pixel_2.x * w as f32, pixel_2.y * h as f32);

            self.image.set_pixel(pixel_0.x as usize, pixel_0.y as usize, Vector3::from_value(1.0));
            self.image.set_pixel(pixel_1.x as usize, pixel_1.y as usize, Vector3::from_value(1.0));
            self.image.set_pixel(pixel_2.x as usize, pixel_2.y as usize, Vector3::from_value(1.0));
        }

        self.image.write_ppm(output_path.into())?;

        Ok(())
    }

    fn viewport_convert(vertex: Vector4<f32>, screen: &Screen) -> Vector2<f32>{
        Vector2::new((vertex.x + 1.0) * screen.w / 2.0, (vertex.y + 1.0) * screen.h / 2.0)
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