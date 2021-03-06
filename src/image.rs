use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::isize;

use cgmath::Array;
use num::clamp;

use crate::Vec2f;
use crate::Vec3f;

pub struct Image {
    width: usize,
    height: usize,
    canvas: Vec<Vec3f>,
    pub depth_canvas: Vec<f32>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let canvas_array_size = width * height;

        Self {
            width,
            height,
            canvas: vec![Vec3f::from_value(0.0); canvas_array_size as usize],
            depth_canvas: vec![f32::INFINITY; canvas_array_size as usize],
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, kd: Vec3f) {
        //たまに物凄く大きな値がくるのを弾いている
        //がそもそもそんなに大きな値が来るのが間違っているのでは？
        if x < 0 || self.width as isize <= x || y < 0 || self.height as isize <= y {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        let target_pixel_index = self.width * y + x;
        self.canvas[target_pixel_index] = kd;
    }

    //Bresenham's line algorithm
    pub fn raster_line(&mut self, p1: Vec2f, p2: Vec2f, kd: Vec3f) {
        let (mut x1, mut y1) = (p1.x as isize, p1.y as isize);
        let (mut x2, mut y2) = (p2.x as isize, p2.y as isize);
        let mut trans = false;

        if (x2 - x1).abs() < (y2 - y1).abs() {
            //swap x1 y1
            std::mem::swap(&mut x1, &mut y1);
            //swap x2 y2
            std::mem::swap(&mut x2, &mut y2);

            trans = true;
        }

        if x1 > x2 {
            //swap x1 x2
            std::mem::swap(&mut x1, &mut x2);
            //swap y1 y2
            std::mem::swap(&mut y1, &mut y2);
        }
        let dx = x2 - x1;
        let dy = y2 - y1;
        let delta = dy.abs() * 2;
        let yd = if dy > 0 { 1 } else { -1 };

        let mut error = 0;
        let mut y = y1;

        for x in x1..=x2 {
            let target_x = if trans { y } else { x };
            let target_y = if trans { x } else { y };
            self.set_pixel(target_x, target_y, kd);
            error += delta;
            if error > dx {
                y += yd;
                error -= dx * 2;
            }
        }
    }

    pub fn write_ppm(&self, output_name: impl Into<Cow<'static, str>>) -> io::Result<()> {
        let output_name: &str = &output_name.into();
        
        let f = File::create(output_name)?;
        let mut writer = BufWriter::new(f);
        
        writer.write_all(b"P3\r\n")?;
        writer.write_all(format!("{} {}\r\n", self.width, self.height).as_bytes())?;
        writer.write_all(b"255\r\n")?;
        
        for j in 0..self.height {
            for i in 0..self.width {
                //通常の画像は左上が(0, 0)だがこのレンダラのキャンバスの作り方的には左下が(0, 0)出ないといけないので上下を反転
                let index = self.width * ((self.height - 1) - j) + i;
                let rgb = self.canvas[index].map(|kd| clamp(kd * 255.0, 0.0, 255.0));

                writer.write_all(format!("{} {} {}\r\n", rgb.x as usize, rgb.y as usize, rgb.z as usize).as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn set_degamma(&mut self) {
        for index in 0..self.canvas.len() {
            self.canvas[index] = self.canvas[index].map(|kd| kd.powf(1.0 / 2.2));
        }
    }
}
