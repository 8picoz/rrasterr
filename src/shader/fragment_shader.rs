use crate::Vec3f;
use crate::Vec4f;

pub fn fragment_shader(normal: Vec3f, p_ndc: Vec4f) -> Vec3f {
    normal_shader(normal, p_ndc)
}

fn normal_shader(normal: Vec3f, p_ndc: Vec4f) -> Vec3f {
    Vec3f::new(normal.x.abs(), normal.y.abs(), normal.z.abs())
}

//directional light