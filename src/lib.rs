use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Vector4;

pub mod image;
pub mod obj;
pub mod triangle;
pub mod camera;
pub mod screen;
pub mod coordinate_helper;
pub mod scene;
pub mod light;
pub mod bounding_box;
pub mod vertex;
pub mod shader;

type Vec2f = Vector2<f32>;
type Vec3f = Vector3<f32>;
type Vec4f = Vector4<f32>;