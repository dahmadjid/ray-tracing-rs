#![allow(dead_code)]

use crate::vec3::{Vec3, Number};
use std::io::Write;
use std::fs::File;
use std::process::Command;

static mut RENDER_COUNT: u32 = 0;

pub struct Image<T> {
    pub width : u32,
    pub height: u32,
    pub data: Vec<Vec3<T>>,
}

impl<T> Image<T> 
where T: Number {
    pub fn new(width: u32, height: u32) -> Image<T> {
        Image{width, height, data: Vec::new()}
    }

    pub fn render(&self ) {
        let filename = unsafe {
            RENDER_COUNT += 1;
            format!("{RENDER_COUNT}.ppm")
        };
        let mut result = format!("P3\n{} {}\n255\n", self.width, self.height);
        self.data.iter().map(|pixel| format!("{pixel}\n")).for_each(|s| {result+=&s;});
        let mut f = File::create(filename).unwrap();
        f.write(result.as_bytes()).unwrap();
    }

    pub fn show(&self) {
        let filename = unsafe {
            if RENDER_COUNT == 0 {
                println!("Not rendered yet");
                return;
            }
            format!("{RENDER_COUNT}.ppm")
        };
        let mut show_ppm = Command::new("display");
        show_ppm.arg(&filename);
        let out = show_ppm.output().expect(&format!("failed to open {}", filename));
        println!("{out:?}");
    }
}

