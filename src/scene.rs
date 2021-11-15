use std::borrow::Cow;
use std::io::Result;

use cgmath::Array;
use cgmath::InnerSpace;
use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Vector4;

use crate::bounding_box::BoundingBox;
use crate::camera::Camera;
use crate::image::Image;
use crate::obj::Obj;
use crate::screen::Screen;
use crate::triangle::Triangle;
use crate::coordinate_helper::projection_matrix;
use crate::coordinate_helper::view_matrix;

type Vec4f = Vector4<f32>;
type Vec2f = Vector2<f32>;

//coordinate_state的なenumを持たせて現在の自分の状態を確認できるようにする
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

    pub fn clipping(&mut self) {
        //それぞれのクリップ面の内側への法線(?)
        let clip_plane_normals = vec![
            Vec4f::new(1.0, 0.0, 0.0, 1.0), //w=-x
            Vec4f::new(-1.0, 0.0, 0.0, 1.0), //w=x
            Vec4f::new(0.0, 1.0, 0.0, 1.0), //w=-y
            Vec4f::new(0.0, -1.0, 0.0, 1.0), //w=y
            Vec4f::new(0.0, 0.0, 1.0, 1.0), //w=-z
            Vec4f::new(0.0, 0.0, -1.0, 1.0), //w=z
            ];
        
        //辺とクリップ面の交差点探索
        let intersect = |v1: Vec4f, v2: Vec4f, d1: f32, d2: f32| -> Vec4f {
            let a = d1 / (d1 - d2);
            ((1.0 - a) * v1 + a * v2).normalize()
        };

        //クリッピング後のtriangles
        let mut triangles = vec![];

        for triangle in &self.obj.triangles {
            //Counter Clockwise Order
            //この時点では頂点は3つしか入っていないが、クリッピング後に増えていく
            let mut polygon = vec![triangle.x, triangle.y, triangle.z];
            for normal in clip_plane_normals.clone() {
                let mut cliped_polygon = vec![];
                for index in 0..polygon.len() {
                    // pick edge
                    let v1 = polygon[index];
                    let v2 = polygon[(index + 1) % polygon.len()];

                    //v1が内側なら d1 > 0 外か辺の上なら d1 <= 0
                    let d1 = v1.dot(normal);
                    //v2が内側なら d2 > 0 外か辺の上なら d2 <= 0
                    let d2 = v2.dot(normal);

                    if d1 > 0.0 {
                        //v1 内側
                        if d2 > 0.0 {
                            //v2 内側
                            cliped_polygon.push(v2);
                        } else {
                            //v2 外側
                            let point = intersect(v1, v2, d1, d2);
                            cliped_polygon.push(point);
                        }
                    } else if d2 > 0.0 {
                        //v1 外側
                        //v2 内側
                        let point = intersect(v1, v2, d1, d2);
                        cliped_polygon.push(v2);
                        cliped_polygon.push(point);
                    }
                }

                polygon = cliped_polygon;
            }

            if !polygon.is_empty() {
                //この三角形からできた頂点で作られる三角形は全てpolygon[0]を含む
                let v1 = polygon[0];
                for idx in 1..(polygon.len() - 1) {
                    let v2 = polygon[idx];
                    let v3 = polygon[idx + 1];
                    let tri = Triangle::new(v1, v2, v3);
                    triangles.push(tri);
                }
            }
        }

        self.obj.triangles = triangles;
    }

    fn viewport_convert(vertex: Vector4<f32>, screen: &Screen) -> Vec2f{
        Vector2::new((vertex.x + 1.0) * screen.w / 2.0, (vertex.y + 1.0) * screen.h / 2.0)
    }

    pub fn rasterize(&mut self) {
        //エッジ関数(CCW)
        //https://dl.acm.org/doi/10.1145/378456.378457
        let edge_func = |a: Vec2f, b: Vec2f, c: Vec2f| {
            let d1 = b - a;
            let d2 = c - a;
            //三次元ベクトルと見做したときの外積のz成分 
            d1.x * d2.y - d1.y * d2.x
        };

        for tri in &self.obj.triangles {
            let (p_0_ndc, p_1_ndc, p_2_ndc) = (tri.x, tri.y, tri.z);

            let pixel_0 = Scene::viewport_convert(tri.x, &self.camera.screen);
            let pixel_1 = Scene::viewport_convert(tri.y, &self.camera.screen);
            let pixel_2 = Scene::viewport_convert(tri.z, &self.camera.screen);

            let (w, h) = self.image.get_size();

            let pixel_0 = Vector2::new(pixel_0.x * w as f32, pixel_0.y * h as f32);
            let pixel_1 = Vector2::new(pixel_1.x * w as f32, pixel_1.y * h as f32);
            let pixel_2 = Vector2::new(pixel_2.x * w as f32, pixel_2.y * h as f32);

            //三角形が逆回りでないかどうか判定
            let denom = edge_func(pixel_0, pixel_1, pixel_2);
            if denom < 0.0 {
                continue;
            }

            let bb = BoundingBox::calc_from_vertexes(vec![pixel_0, pixel_1, pixel_2]);

            let (x_min, x_max) = (bb.min.x as usize, bb.max.x as usize);
            let (y_min, y_max) = (bb.min.y as usize, bb.max.y as usize);

            //BBを使ってピクセルを限定
            for x in x_min..x_max {
                for y in y_min..y_max {
                    //offsetを取らないとそのピクセルの左下部分から始まることになり
                    //p1, p2, p3と重なってしまう
                    let p = Vec2f::new(x as f32, y as f32) + Vec2f::from_value(0.5);

                    //エッジ
                    let b0 = edge_func(pixel_1, pixel_2, p);
                    let b1 = edge_func(pixel_2, pixel_0, p);
                    let b2 = edge_func(pixel_0, pixel_1, p);

                    //Winding Order is CCW
                    // b1 < 0.0 && b2 < 0.0 && b3 < 0.0 とすれば逆回りの三角形の描画もできる
                    if b0 > 0.0 && b1 > 0.0 && b2 > 0.0 {
                        //?
                        let b0 = b0 / denom;
                        let b1 = b1 / denom;
                        let b2 = b2 / denom;

                        //pのデバイス座標系への変換？
                        let p_ndc = b0 * p_0_ndc + b1 * p_1_ndc + b2 * p_2_ndc;

                        if self.image.depth_canvas[w * y + x] < p_ndc.z {
                            continue;
                        }
                        self.image.depth_canvas[w * y + x] = p_ndc.z;
                        //directional light未実装
                        self.image.set_pixel(x as isize, y as isize, Vector3::<f32>::from_value(1.0));
                    }
                }
            }
        }
    }

    //wire frame
    pub fn render_line(&mut self) {
        for tri in &self.obj.triangles {
            let pixel_0 = Scene::viewport_convert(tri.x, &self.camera.screen);
            let pixel_1 = Scene::viewport_convert(tri.y, &self.camera.screen);
            let pixel_2 = Scene::viewport_convert(tri.z, &self.camera.screen);

            let (w, h) = self.image.get_size();

            let pixel_0 = Vector2::new(pixel_0.x * w as f32, pixel_0.y * h as f32);
            let pixel_1 = Vector2::new(pixel_1.x * w as f32, pixel_1.y * h as f32);
            let pixel_2 = Vector2::new(pixel_2.x * w as f32, pixel_2.y * h as f32);

            self.image.raster_line(pixel_0, pixel_1, Vector3::from_value(1.0));
            self.image.raster_line(pixel_1, pixel_2, Vector3::from_value(1.0));
            self.image.raster_line(pixel_2, pixel_0, Vector3::from_value(1.0));
        }
    }

    pub fn render_vertex(&mut self) {
        for tri in &self.obj.triangles {
            let pixel_0 = Scene::viewport_convert(tri.x, &self.camera.screen);
            let pixel_1 = Scene::viewport_convert(tri.y, &self.camera.screen);
            let pixel_2 = Scene::viewport_convert(tri.z, &self.camera.screen);

            let (w, h) = self.image.get_size();

            let pixel_0 = Vector2::new(pixel_0.x * w as f32, pixel_0.y * h as f32);
            let pixel_1 = Vector2::new(pixel_1.x * w as f32, pixel_1.y * h as f32);
            let pixel_2 = Vector2::new(pixel_2.x * w as f32, pixel_2.y * h as f32);

            self.image.set_pixel(pixel_0.x as isize, pixel_0.y as isize, Vector3::from_value(1.0));
            self.image.set_pixel(pixel_1.x as isize, pixel_1.y as isize, Vector3::from_value(1.0));
            self.image.set_pixel(pixel_2.x as isize, pixel_2.y as isize, Vector3::from_value(1.0));
        }
    }

    pub fn generate_image(&self, output_path: impl Into<Cow<'static , str>>) -> Result<()> {
        self.image.write_ppm(output_path.into())
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