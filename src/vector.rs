use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
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
        Self { x: 0.0, y: 0.0 }
    }
    pub fn distance(a: Vec2, b: Vec2) -> f64 {
        (a - b).length()
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    // operator functions for js
    pub fn add_num(&self, rhs: f64) -> Self {
        *self + rhs
    }
    pub fn add_vec(&self, rhs: Vec2) -> Self {
        *self + rhs
    }
    pub fn mult_num(&self, rhs: f64) -> Self {
        *self * rhs
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<f64> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<f64> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
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
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
