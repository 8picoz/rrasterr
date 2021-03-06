use cgmath::InnerSpace;

use crate::Vec3f;
use crate::screen::Screen;

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
        let basis_y = basis_z.cross(basis_x);

        println!("{:?}, {:?}, {:?}", basis_x, basis_y, basis_z);

        Self { position, endpoint, up_direction, screen, basis_x, basis_y, basis_z }
    }
}