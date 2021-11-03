use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use cgmath::Array;
use cgmath::Vector3;
use num::clamp;

pub struct Image {
    width: usize,
    height: usize,
    canvas: Vec<Vector3<f32>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let canvas_array_size = 3 * width * height;

        Self { width, height, canvas: vec![Vector3::from_value(0.0); canvas_array_size as usize] }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, kd: Vector3<f32>) {
        let target_pixel_index = x + self.width * y;

        self.canvas[target_pixel_index] = kd;
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
                let index = i + self.width * j;
                let rgb = self.canvas[index].map(|kd| { clamp(kd * 255.0, 255.0, 0.0) });

                writer.write_all(format!("{} {} {}\r\n", rgb.x, rgb.y, rgb.z).as_bytes())?;
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
