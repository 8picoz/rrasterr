use cgmath::Matrix4;

use crate::camera::Camera;

//TODO: なんかもっと良いファイル名

pub fn view_matrix(camera: Camera) -> Matrix4<f32> {
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