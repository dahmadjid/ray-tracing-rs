#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display};

use rand::Rng;

use crate::{mat3::Mat3, quat::Quat};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }
}

impl Vec3<f64> {
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> Self {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        *self
    }

    pub fn normalize(&self) -> Vec3<f64> {
        let one_over_length = 1.0 / self.length();
        Vec3::new(
            self.x * one_over_length, 
            self.y * one_over_length, 
            self.z * one_over_length,
        )
    }

    pub fn floor(&self) -> Vec3<i64> {
        Vec3{
            x: self.x.floor() as i64,
            y: self.y.floor() as i64, 
            z: self.z.floor() as i64,
        } 
    }

    pub fn ceil(&self) -> Vec3<i64> {
        Vec3{
            x: self.x.ceil() as i64,
            y: self.y.ceil() as i64, 
            z: self.z.ceil() as i64,
        } 
    }

    pub fn random() -> Vec3<f64> {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn shift(&mut self, scalar: f64) -> Vec3<f64> {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
        *self
    }

    pub fn rotate_yaw_matrix_world(&mut self, degrees_angle: f64) -> Self{
        let cos_angle = degrees_angle.to_radians().cos();
        let sin_angle = degrees_angle.to_radians().sin();
        let rotation_matrix = Mat3::new([
            cos_angle, 0., sin_angle,
            0., 1., 0.,
            -sin_angle, 0., cos_angle,
        ]);

        let res = rotation_matrix.vec_mul(self);
        self.x = res.x;
        self.y = res.y;
        self.z = res.z;
        *self
    }

    pub fn rotate_pitch_matrix_world(&mut self, degrees_angle: f64) -> Self{
        let cos_angle = degrees_angle.to_radians().cos();
        let sin_angle = degrees_angle.to_radians().sin();
        let rotation_matrix = Mat3::new([
            1., 0., 0.,
            0., cos_angle, -sin_angle,
            0., sin_angle, cos_angle,
        ]);

        let res = rotation_matrix.vec_mul(self);
        self.x = res.x;
        self.y = res.y;
        self.z = res.z;
        *self
    }

    pub fn rotate_roll_matrix_world(&mut self, degrees_angle: f64) -> Self{
        let cos_angle = degrees_angle.to_radians().cos();
        let sin_angle = degrees_angle.to_radians().sin();
        let rotation_matrix = Mat3::new([
            cos_angle, -sin_angle, 0.,
            sin_angle, cos_angle, 0.,
            0., 0., 1., 
        ]);

        let result = rotation_matrix.vec_mul(self);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
        *self
    }

    pub fn rotate(&mut self, rotation_quaternion: Quat<f64>) -> Self {
        let quat_to_rotate = Quat{w: 0.0, v: *self};
        let result = rotation_quaternion.cross(&quat_to_rotate).cross(&-rotation_quaternion);
        self.x = result.v.x;
        self.y = result.v.y;
        self.z = result.v.z;
        *self
    }

}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl From<Vec3<f64>> for Vec3<u8> {
    fn from(v: Vec3<f64>) -> Vec3<u8> {
        Vec3::<u8>{x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec3<i64>> for Vec3<u8> {
    fn from(v: Vec3<i64>) -> Vec3<u8> {
        Vec3::<u8>{x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec3<u8>> for Vec3<f64> {
    fn from(v: Vec3<u8>) -> Vec3<f64> {
        Vec3::<f64>{x: v.x as f64, y: v.y as f64, z: v.z as f64}
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use super::*;

    #[test]
    fn dot() {
        let a = Vec3::new(1.5, 2.5, 10.);
        let b = Vec3::new(20.5, 3., 3.);

        let c = a.dot(&b);

        assert_eq!(c, 68.25)
    }
    #[test]
    fn cross() {
        let a = Vec3::new(1.5, 2.5, 10.);
        let b = Vec3::new(20.5, 3., 3.);

        let c = a.cross(&b);

        assert_eq!(c, Vec3::new(-22.5, 200.5, -46.75))
    }

    #[test]
    fn length() {
        let a = Vec3::new(5.0, 3.0, 1.0);
        let l = a.length();

        assert_eq!(l, 35.0f64.sqrt())
    }

    #[test]
    fn rotation_quat() {
        let a = Vec3::new(1., 0., 0.);
        let axis = Vec3::new(1., 1., 0.).normalize();
        let rotated_by_quat = a.clone().rotate(Quat::angle_axis(45.0f64.to_radians(), axis));

        // rotation matrix of 45 degrees from the internet by the (1, 1, 0) axis.
        let rotation_matrix = Mat3::new([
            0.8535534,  0.1464466,  0.5000000,
            0.1464466,  0.8535534, -0.500000,
            -0.5000000,  0.5000000,  0.7071068 
        ]);

        
        let rotated_by_matrix = rotation_matrix.vec_mul(&a);
        assert_float_absolute_eq!(rotated_by_matrix.x, rotated_by_quat.x, 0.0000001);
        assert_float_absolute_eq!(rotated_by_matrix.y, rotated_by_quat.y, 0.0000001);
        assert_float_absolute_eq!(rotated_by_matrix.z, rotated_by_quat.z, 0.0000001);

    }
}