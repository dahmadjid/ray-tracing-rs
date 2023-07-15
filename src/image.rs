#![allow(dead_code)]

use crate::vec3::{Vec3, Number};
use std::io::Write;
use std::fs::File;
use std::process::Command;


pub struct Image<T> {
    pub width : u32,
    pub height: u32,
    pub filename: String,
    pub data: Vec<Vec3<T>>,
}

impl<T> Image<T> 
where T: Number {
    pub fn new(width: u32, height: u32) -> Image<T> {
        Image{width, height, filename:String::new(), data: Vec::new()}
    }

    pub fn render(&mut self, file: &'static str ) {
        let mut result = format!("P3\n{} {}\n255\n", self.width, self.height);
        self.data.iter().map(|pixel| format!("{pixel}\n")).for_each(|s| {result+=&s;});
        let mut f = File::create(file).unwrap();
        self.filename = String::from(file); 
        f.write(result.as_bytes()).unwrap();
    }

    pub fn show(&self) {
        if self.filename == "" {
            println!("Not rendered yet");
            return;
        }
        let mut show_ppm = Command::new("display");
        show_ppm.arg(&self.filename);
        let out = show_ppm.output().expect(&format!("failed to open {}", self.filename));
        println!("{out:?}");
    }
}

