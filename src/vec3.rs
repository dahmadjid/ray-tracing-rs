#![allow(dead_code)]

use std::ops::{Add, Div, Sub, Mul, Neg};
use std::fmt::Display;
use num::{NumCast, Num};
pub trait Number: Num + Neg<Output=Self> + NumCast + Copy + Num + Display {}
impl<T: Num + NumCast + Copy + Neg<Output=T> + Display> Number for T {}

#[derive(Debug, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> 
where T: Number {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y - rhs.x,
        )
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).to_f64().unwrap_or(-1.0).sqrt();
    }

    pub fn scale(&self, factor: T) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub fn normalize(&self) -> Vec3<f64> {
        let length = self.length();
        Vec3::new(
            self.x.to_f64().unwrap_or(0.) / length, 
            self.y.to_f64().unwrap_or(0.) / length, 
            self.z.to_f64().unwrap_or(0.) / length,
        )
    }

    pub fn floor(&self) -> Vec3<i64> {
        Vec3::new(
            self.x.to_f64().unwrap_or(0.).floor() as i64,
            self.y.to_f64().unwrap_or(0.).floor() as i64, 
            self.z.to_f64().unwrap_or(0.).floor() as i64,
        ) 
    }

    pub fn ceil(&self) -> Vec3<i64> {
        Vec3::new(
            self.x.to_f64().unwrap_or(0.).ceil() as i64,
            self.y.to_f64().unwrap_or(0.).ceil() as i64, 
            self.z.to_f64().unwrap_or(0.).ceil() as i64,
        ) 
    }
}


impl<T> Add for Vec3<T> 
where T: Number {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where T: Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T> Mul for Vec3<T> 
where T: Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl<T> Div for Vec3<T> 
where T: Number {
    type Output = Vec3<T>;
    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output{x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl<T> Neg for Vec3<T> 
where T: Number {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Display for Vec3<T>
where T: Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

