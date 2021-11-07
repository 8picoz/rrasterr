use cgmath::{Array, Vector3};
use clap::{Arg, App};
use mitsuba_raster::obj::Obj;

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

    let target_obj = Obj::new(file_path, Vector3::from_value(0.0));

    
}
