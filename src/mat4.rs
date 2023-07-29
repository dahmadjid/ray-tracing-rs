#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display};
use crate::vec4::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Mat4<T> {
    pub data : [T; 16],
}

impl<T: Copy> Mat4<T> {
    pub fn new(data: [T; 16]) -> Self {
        Self{data: data}
    }
    pub fn get(&self, y: usize, x: usize) -> T {
        self.data[y * 4 + x]
    }
}

impl<T: Copy + Add<Output=T> + Mul<Output=T>> Mat4<T> {
    pub fn mat_mul(&self, other: &Self) -> Self {
        Self::new(
            [
                self.get(0, 0) * other.get(0, 0) + self.get(0, 1) * other.get(1, 0) + self.get(0, 2) * other.get(2, 0) + self.get(0, 3) * other.get(3, 0),
                self.get(0, 0) * other.get(0, 1) + self.get(0, 1) * other.get(1, 1) + self.get(0, 2) * other.get(2, 1) + self.get(0, 3) * other.get(3, 1),
                self.get(0, 0) * other.get(0, 2) + self.get(0, 1) * other.get(1, 2) + self.get(0, 2) * other.get(2, 2) + self.get(0, 3) * other.get(3, 2),
                self.get(0, 0) * other.get(0, 3) + self.get(0, 1) * other.get(1, 3) + self.get(0, 2) * other.get(2, 3) + self.get(0, 3) * other.get(3, 3),

                self.get(1, 0) * other.get(0, 0) + self.get(1, 1) * other.get(1, 0) + self.get(1, 2) * other.get(2, 0) + self.get(1, 3) * other.get(3, 0),
                self.get(1, 0) * other.get(0, 1) + self.get(1, 1) * other.get(1, 1) + self.get(1, 2) * other.get(2, 1) + self.get(1, 3) * other.get(3, 1),
                self.get(1, 0) * other.get(0, 2) + self.get(1, 1) * other.get(1, 2) + self.get(1, 2) * other.get(2, 2) + self.get(1, 3) * other.get(3, 2),
                self.get(1, 0) * other.get(0, 3) + self.get(1, 1) * other.get(1, 3) + self.get(1, 2) * other.get(2, 3) + self.get(1, 3) * other.get(3, 3),
                
                self.get(2, 0) * other.get(0, 0) + self.get(2, 1) * other.get(1, 0) + self.get(2, 2) * other.get(2, 0) + self.get(2, 3) * other.get(3, 0),
                self.get(2, 0) * other.get(0, 1) + self.get(2, 1) * other.get(1, 1) + self.get(2, 2) * other.get(2, 1) + self.get(2, 3) * other.get(3, 1),
                self.get(2, 0) * other.get(0, 2) + self.get(2, 1) * other.get(1, 2) + self.get(2, 2) * other.get(2, 2) + self.get(2, 3) * other.get(3, 2),
                self.get(2, 0) * other.get(0, 3) + self.get(2, 1) * other.get(1, 3) + self.get(2, 2) * other.get(2, 3) + self.get(2, 3) * other.get(3, 3),

                self.get(3, 0) * other.get(0, 0) + self.get(3, 1) * other.get(1, 0) + self.get(3, 2) * other.get(2, 0) + self.get(3, 3) * other.get(3, 0),
                self.get(3, 0) * other.get(0, 1) + self.get(3, 1) * other.get(1, 1) + self.get(3, 2) * other.get(2, 1) + self.get(3, 3) * other.get(3, 1),
                self.get(3, 0) * other.get(0, 2) + self.get(3, 1) * other.get(1, 2) + self.get(3, 2) * other.get(2, 2) + self.get(3, 3) * other.get(3, 2),
                self.get(3, 0) * other.get(0, 3) + self.get(3, 1) * other.get(1, 3) + self.get(3, 2) * other.get(2, 3) + self.get(3, 3) * other.get(3, 3),
             
            ]
        )
    }

    pub fn vec_mul(&self, other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(
            self.get(0, 0) * other.w + self.get(0, 1) * other.x + self.get(0, 2) * other.y + self.get(0, 3) * other.z,
            self.get(1, 0) * other.w + self.get(1, 1) * other.x + self.get(1, 2) * other.y + self.get(1, 3) * other.z,
            self.get(2, 0) * other.w + self.get(2, 1) * other.x + self.get(2, 2) * other.y + self.get(2, 3) * other.z,
            self.get(3, 0) * other.w + self.get(3, 1) * other.x + self.get(3, 2) * other.y + self.get(3, 3) * other.z,
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
impl<T: Display + Copy + num::Num + PartialOrd> Display for Mat4<T> {
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
        let m9 = right_pad(format!("{}", self.data[9]), biggest_str_len, " ");
        let m10 = right_pad(format!("{}", self.data[10]), biggest_str_len, " ");
        let m11 = right_pad(format!("{}", self.data[11]), biggest_str_len, " ");
        let m12 = right_pad(format!("{}", self.data[12]), biggest_str_len, " ");
        let m13 = right_pad(format!("{}", self.data[13]), biggest_str_len, " ");
        let m14 = right_pad(format!("{}", self.data[14]), biggest_str_len, " ");
        let m15 = right_pad(format!("{}", self.data[15]), biggest_str_len, " ");
        write!(f, "{m0} {m1} {m2} {m3}\n{m4} {m5} {m6} {m7}\n{m8} {m9} {m10} {m11}\n{m12} {m13} {m14} {m15}\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_mul() {
        let a = Mat4::new([
            1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,
        ]);
        let b = Mat4::new([
            1,2,3,4,5,6,7,8,8,7,6,5,4,3,2,1,
        ]);

        let c = a.mat_mul(&b);

        assert_eq!(c.data, [51, 47, 43, 39, 123, 119, 115, 111, 51, 47, 43, 39, 123, 119, 115, 111])
    }
    #[test]
    fn vec_mul() {
        let a = Mat4::new([
            1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,
        ]);

        let b = Vec4::new(1, 2, 3, 4);

        let c = a.vec_mul(&b);

        assert_eq!(c, Vec4::new(30, 70, 30, 70));
    }
}