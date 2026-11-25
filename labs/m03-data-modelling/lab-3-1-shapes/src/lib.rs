#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 3.1 — Shapes.
//!
//! After the assessment passes, add a `Square { side: f64 }` variant and run
//! `cargo build`: the compiler lists every `match` you must update. That
//! exhaustiveness check is the refactoring superpower of Rust enums.

use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        todo!("lab-3-1-shapes: implement me")
    }

    pub fn perimeter(&self) -> f64 {
        todo!("lab-3-1-shapes: implement me")
    }

    /// All dimensions positive, and triangles satisfy the triangle inequality.
    pub fn is_valid(&self) -> bool {
        todo!("lab-3-1-shapes: implement me")
    }

    /// Multiply every dimension by `k`, in place. Note `&mut self`.
    pub fn scale(&mut self, k: f64) {
        todo!("lab-3-1-shapes: implement me")
    }

    /// "circle", "rect" or "triangle".
    pub fn kind(&self) -> &'static str {
        todo!("lab-3-1-shapes: implement me")
    }
}

/// The shape with the largest area, or `None` for an empty slice.
/// Note the return type borrows from the input — no clone needed.
pub fn largest(shapes: &[Shape]) -> Option<&Shape> {
    todo!("lab-3-1-shapes: implement me")
}
