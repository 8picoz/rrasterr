use cgmath::InnerSpace;
use cgmath::Vector3;
use crate::screen::Screen;

type Vec3f = Vector3<f32>;

pub struct Camera {
    pub position: Vec3f,
    pub endpoint: Vec3f,
    pub up_direction: Vec3f,
    pub screen: Screen,
    //basis
    pub basis_x: Vec3f,
    pub basis_y: Vec3f,
    pub basis_z: Vec3f,
}

impl Camera {
    pub fn new(position: Vec3f, endpoint: Vec3f, up_direction: Vec3f, screen: Screen) -> Self {
        let basis_z = (position - endpoint).normalize();
        let basis_x = basis_z.cross(up_direction);
        let basis_y = basis_z.cross(basis_x);

        Self { position, endpoint, up_direction, screen, basis_x, basis_y, basis_z }
    }
}