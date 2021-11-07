use cgmath::{Array, Vector3};
use clap::{Arg, App};
use mitsuba_raster::{camera::Camera, obj::Obj, screen::Screen};

fn main() {
    let matched = App::new("mitsuba-raster")
        .version("0.1")
        .author("tata <8picoz@tata.pw>")
        .about("rasterizer")
        .arg(Arg::with_name("file_path")
            .short("f")
            .long("file_path")
            .value_name("file_path")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let file_path = matched.value_of("file_path").unwrap();

    let screen = Screen::new(2.0, 10.0, 1.0, 1.0);
    let camera = Camera::new(
        Vector3::new(0.0, 6.0, 28.0), 
        Vector3::new(-0.2, 1.6, 0.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        screen);

    let target_obj = Obj::new(file_path, Vector3::from_value(0.0));
    
    
}
