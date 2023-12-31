// TODO: change all length/distance comparisons to squared length/distance

use std::{fmt::Debug, ops::*};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
#[wasm_bindgen]
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn from(val: f64) -> Self {
        Self::new(val, val)
    }
    pub fn rand_01() -> Self {
        Self::new(js_sys::Math::random(), js_sys::Math::random())
    }
    pub fn to_str(&self) -> String {
        format!("Vec2({:.2}, {:.2})", self.x, self.y)
    }

    /**
    returns vector with y = 0
    */
    pub fn keep_x(&self) -> Self {
        Self::new(self.x, 0.0)
    }
    /**
    returns vector with x = 0
    */
    pub fn keep_y(&self) -> Self {
        Self::new(0.0, self.y)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn distance_squared(&self, other: &Vec2) -> f64 {
        (*self - *other).length_squared()
    }
    pub fn distance(&self, other: &Vec2) -> f64 {
        self.distance_squared(other).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
    pub fn rotate(&self, radians: f64) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
    pub fn clamp_length(&self, min: f64, max: f64) -> Self {
        let len = self.length();
        if len < min {
            self.normalize() * min
        } else if len > max {
            self.normalize() * max
        } else {
            *self
        }
    }

    // operator functions for js
    // all vec2 have to be borrowed so the
    // wasm pointer isnt destroyed
    pub fn neg(&self) -> Self {
        -*self
    }
    pub fn add_num(&self, rhs: f64) -> Self {
        *self + rhs
    }
    pub fn add_vec(&self, rhs: &Vec2) -> Self {
        *self + *rhs
    }
    pub fn sub_num(&self, rhs: f64) -> Self {
        *self - rhs
    }
    pub fn sub_vec(&self, rhs: &Vec2) -> Self {
        *self - *rhs
    }
    pub fn mul_num(&self, rhs: f64) -> Self {
        *self * rhs
    }
    pub fn mul_vec(&self, rhs: &Vec2) -> Self {
        *self * *rhs
    }
    pub fn div_num(&self, rhs: f64) -> Self {
        *self / rhs
    }
}
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<f64> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<f64> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs
    }
}
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

pub trait Position {
    fn pos(&self) -> Vec2;
    fn x(&self) -> f64 {
        self.pos().x
    }
    fn y(&self) -> f64 {
        self.pos().y
    }
}
impl Position for Vec2 {
    fn pos(&self) -> Vec2 {
        *self
    }
}

#[test]
fn test_vec2_eq() {
    assert_eq!(Vec2::zero(), Vec2::zero());
    assert_eq!(Vec2::from(1.0), Vec2::from(1.0));
    assert_eq!(Vec2::from(43.13), Vec2::from(43.13));

    assert_ne!(Vec2::from(25.4), Vec2::from(50.80000000001) / 2.0);
    assert_ne!(Vec2::new(9.4, 5.6), Vec2::new(12.6, 5.7));
}
