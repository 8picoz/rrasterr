use std::borrow::Cow;

use cgmath::Vector3;
use clap::{Arg, App};
use rrasterr::{camera::Camera, image::Image, obj::Obj, scene::Scene, screen::Screen};

type Vec3f = Vector3<f32>;

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

    let screen = Screen::new(2.0, 1000.0, 0.5, -0.5, 0.5, -0.5);
    let camera = Camera::new(
        //マイナス方向に進めると上方向にカメラが移動するということは
        Vec3f::new(0.0, 5.0, 8.0), 
        Vec3f::new(0.0, 1.0, 0.0), 
        Vec3f::new(0.0, 1.0, 0.0), 
        screen);

    let target_obj = Obj::new(file_path, Vec3f::new(0.0, 0.0, 0.0));
    
    let mut scene = Scene::new(image, camera, target_obj);

    //モデル変換
    //原点から移動させるつもりがない限り入らない

    //ビュー変換
    //カメラ座標系
    scene.view_convert();
    println!("view convert");
    //投影変換
    //クリップ座標系
    scene.projection_convert();
    println!("projection convert");
    //クリッピング
    //Sutherland-Hodgman
    scene.clipping();
    println!("clipping");
    //Perspective Division
    //デバイス座標系
    scene.perspective_division();
    println!("perspective division");

    //レンダリング
    if render_vertex {
        scene.render_vertex();
    }
    if render_line {
        scene.render_line();
    }
    if render_raster {
        scene.rasterize(false);
    }
    println!("rendering");
    
    //画像保存
    scene.generate_image("./output.ppm").expect("failed to generate image");
    println!("generate image");
}