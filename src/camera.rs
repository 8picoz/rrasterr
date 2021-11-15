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
        //左手座標系
        let basis_z = (position - endpoint).normalize();
        let basis_x = up_direction.cross(basis_z);
        let basis_y = basis_x.cross(basis_z);

        //println!("{:?}, {:?}, {:?}", basis_x, basis_y, basis_z);

        Self { position, endpoint, up_direction, screen, basis_x, basis_y, basis_z }
    }
}