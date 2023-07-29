#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat3<T> {
    pub data : [T; 9],
}

impl<T: Copy> Mat3<T> {
    pub fn new(data: [T; 9]) -> Self {
        Self{data: data}
    }
    pub fn get(&self, y: usize, x: usize) -> T {
        self.data[y * 3 + x]
    }
}

impl<T: Copy + Add<Output=T> + Mul<Output=T>> Mat3<T> {
    pub fn mat_mul(&self, other: &Self) -> Self {
        Self::new(
            [
                self.get(0, 0) * other.get(0, 0) + self.get(0, 1) * other.get(1, 0) + self.get(0, 2) * other.get(2, 0),
                self.get(0, 0) * other.get(0, 1) + self.get(0, 1) * other.get(1, 1) + self.get(0, 2) * other.get(2, 1),
                self.get(0, 0) * other.get(0, 2) + self.get(0, 1) * other.get(1, 2) + self.get(0, 2) * other.get(2, 2),
                self.get(1, 0) * other.get(0, 0) + self.get(1, 1) * other.get(1, 0) + self.get(1, 2) * other.get(2, 0),
                self.get(1, 0) * other.get(0, 1) + self.get(1, 1) * other.get(1, 1) + self.get(1, 2) * other.get(2, 1),
                self.get(1, 0) * other.get(0, 2) + self.get(1, 1) * other.get(1, 2) + self.get(1, 2) * other.get(2, 2),
                self.get(2, 0) * other.get(0, 0) + self.get(2, 1) * other.get(1, 0) + self.get(2, 2) * other.get(2, 0),
                self.get(2, 0) * other.get(0, 1) + self.get(2, 1) * other.get(1, 1) + self.get(2, 2) * other.get(2, 1),
                self.get(2, 0) * other.get(0, 2) + self.get(2, 1) * other.get(1, 2) + self.get(2, 2) * other.get(2, 2),
             
            ]
        )
    }

    pub fn vec_mul(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.get(0, 0) * other.x + self.get(0, 1) * other.y + self.get(0, 2) * other.z,
            self.get(1, 0) * other.x + self.get(1, 1) * other.y + self.get(1, 2) * other.z,
            self.get(2, 0) * other.x + self.get(2, 1) * other.y + self.get(2, 2) * other.z,
        )
    }
}


fn right_pad(mut s:String, width: usize, c: &str) -> String {
    if s.len() >= width {
        s
    } else {
        for _ in 0..(width - s.len()) {
            s = s + c;
        } 
        s
    }
}
impl<T: Display + Copy + num::Num + PartialOrd> Display for Mat3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut biggest = T::zero();
        for i in 0..9 {
            if self.data[i] > biggest {
                biggest = self.data[i]
            }
        }
        let biggest_str_len = format!("{}", biggest).len();
        let m0 = right_pad(format!("{}", self.data[0]), biggest_str_len, " ");
        let m1 = right_pad(format!("{}", self.data[1]), biggest_str_len, " ");
        let m2 = right_pad(format!("{}", self.data[2]), biggest_str_len, " ");
        let m3 = right_pad(format!("{}", self.data[3]), biggest_str_len, " ");
        let m4 = right_pad(format!("{}", self.data[4]), biggest_str_len, " ");
        let m5 = right_pad(format!("{}", self.data[5]), biggest_str_len, " ");
        let m6 = right_pad(format!("{}", self.data[6]), biggest_str_len, " ");
        let m7 = right_pad(format!("{}", self.data[7]), biggest_str_len, " ");
        let m8 = right_pad(format!("{}", self.data[8]), biggest_str_len, " ");
        write!(f, "{m0} {m1} {m2}\n{m3} {m4} {m5}\n{m6} {m7} {m8}\n")
    }
}