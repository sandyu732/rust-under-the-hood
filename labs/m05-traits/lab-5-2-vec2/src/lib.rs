#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 5.2 — Vector math with operator overloading and conversion traits.
//! Goal: `let v: Vec2 = (1.0, 2.0).into(); let w = v + v * 3.0 - Vec2::default();`

use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn length(self) -> f64 {
        todo!("lab-5-2-vec2: implement me")
    }

    pub fn dot(self, other: Vec2) -> f64 {
        todo!("lab-5-2-vec2: implement me")
    }

    /// Unit vector in the same direction; `None` for the zero vector.
    pub fn normalized(self) -> Option<Vec2> {
        todo!("lab-5-2-vec2: implement me")
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        todo!("lab-5-2-vec2: implement me")
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        todo!("lab-5-2-vec2: implement me")
    }
}

/// `v * 2.0`
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        todo!("lab-5-2-vec2: implement me")
    }
}

/// `2.0 * v` — implementing a std trait for a std type (f64) is allowed
/// because `Vec2` (a type parameter of the impl) is local.
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        todo!("lab-5-2-vec2: implement me")
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        todo!("lab-5-2-vec2: implement me")
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        todo!("lab-5-2-vec2: implement me")
    }
}

/// `(1, 2.5)` — use `{}` for each f64.
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("lab-5-2-vec2: implement me")
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        todo!("lab-5-2-vec2: implement me")
    }
}
