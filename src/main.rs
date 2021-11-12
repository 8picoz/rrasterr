use std::borrow::Cow;

use cgmath::{Array, Vector3};
use clap::{Arg, App};
use rrasterr::{camera::Camera, image::Image, obj::Obj, scene::Scene, screen::Screen};

fn main() {
    let matched = App::new("rrasterr")
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

    rasterize(matched.value_of("file_path").unwrap());
}

fn rasterize<'a>(file_path: impl Into<Cow<'a, str>>) {

    //セットアップ
    let file_path = file_path.into();

    let image = Image::new(512, 512);

    let screen = Screen::new(1.0, 100.0, 1.0, 1.0);
    let camera = Camera::new(
        Vector3::new(-0.5, 1.5, 2.0), 
        Vector3::new(-0.5, 1.5, 0.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        screen);

    let target_obj = Obj::new(file_path, Vector3::from_value(0.0));
    
    let mut scene = Scene::new(image, camera, target_obj);

    //モデル変換
    //原点から移動させるつもりがない限り入らない

    //ビュー変換
    //カメラ座標系
    scene.as_mut().view_convert();
    println!("view convert");
    //投影変換
    //クリップ座標系
    scene.as_mut().projection_convert();
    println!("projection convert");
    //クリッピング
    //Sutherland-Hodgman
    scene.as_mut().clipping();
    println!("clipping");
    //Perspective Division
    //デバイス座標系
    scene.as_mut().perspective_division();
    println!("perspective division");

    //画像保存
    scene.generate_image("./output.ppm").expect("failed output image");
    println!("generate image");
}