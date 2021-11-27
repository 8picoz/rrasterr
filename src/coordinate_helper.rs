use cgmath::Matrix4;

use crate::camera::Camera;
use crate::screen::Screen;

type Mat4f = Matrix4<f32>;

pub fn view_matrix(camera: &Camera) -> Mat4f {
    let translation_matrix = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -camera.position.x, -camera.position.y, -camera.position.z, 1.0,
    );

    let rotation_matrix = Matrix4::new(
        camera.basis_x.x, camera.basis_y.x, camera.basis_z.x, 0.0,
        camera.basis_x.y, camera.basis_y.y, camera.basis_z.y, 0.0,
        camera.basis_x.z, camera.basis_y.z, camera.basis_z.z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    rotation_matrix * translation_matrix
}

pub fn projection_matrix(screen: &Screen) -> Mat4f {
    Matrix4::new(
        (2.0 * screen.near_clip_distance) / screen.width, 0.0, 0.0, 0.0,
        0.0, (2.0 * screen.near_clip_distance) / screen.height, 0.0, 0.0,
        (screen.right + screen.left) / (screen.right - screen.left), (screen.top + screen.bottom) / (screen.top - screen.bottom), - ((screen.far_clip_distance + screen.near_clip_distance) / (screen.far_clip_distance - screen.near_clip_distance)), -1.0,
        0.0, 0.0, - ((2.0 * screen.far_clip_distance * screen.near_clip_distance) / (screen.far_clip_distance - screen.near_clip_distance)), 0.0
    )
}