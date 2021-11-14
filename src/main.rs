use std::borrow::Cow;

use cgmath::{Array, Vector3};
use clap::{Arg, App};
use rrasterr::{camera::Camera, image::Image, obj::Obj, scene::Scene, screen::Screen};

fn main() {
    let file_path_flag_name = "file_path";
    let vertex_flag_name = "vertex";
    let line_flag_name = "line";
    let rasterize_flag_name = "rasterize";

    let matched = App::new("rrasterr")
        .version("0.1")
        .author("tata <8picoz@tata.pw>")
        .about("rasterizer")
        .arg(Arg::with_name(file_path_flag_name)
            .short("f")
            .long(file_path_flag_name)
            .value_name(file_path_flag_name)
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name(vertex_flag_name)
            .short("v")
            .long(vertex_flag_name)
            .help("rendering vertex")
        )
        .arg(Arg::with_name(line_flag_name)
            .short("l")
            .long(line_flag_name)
            .help("rendering line")
        )
        .arg(Arg::with_name(rasterize_flag_name)
            .short("r")
            .long(rasterize_flag_name)
            .help("rendering with rasterization")
        )
        .get_matches();

    rasterize(
    matched.value_of(file_path_flag_name).unwrap(), 
    matched.is_present(vertex_flag_name), 
    matched.is_present(line_flag_name),
    matched.is_present(rasterize_flag_name)
    );
}

fn rasterize<'a>(file_path: impl Into<Cow<'a, str>>, render_vertex: bool, render_line: bool, render_raster: bool) {

    //セットアップ
    let file_path = file_path.into();

    let image = Image::new(512, 512);

    let screen = Screen::new(1.0, 100.0, 1.0, 1.0);
    let camera = Camera::new(
        Vector3::new(-0.5, 1.5, 5.0), 
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

    //レンダリング
    if render_vertex {
        scene.render_vertex();
    }
    if render_line {
        scene.render_line();
    }
    if render_raster {
        scene.rasterize();
    }
    println!("rendering");
    
    //画像保存
    scene.generate_image("./output.ppm").expect("failed to generate image");
    println!("generate image");
}