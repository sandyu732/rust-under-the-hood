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
        // SOLUTION-START
        match *self {
            Shape::Circle { r } => PI * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Triangle { a, b, c } => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
        // SOLUTION-END
    }

    pub fn perimeter(&self) -> f64 {
        // SOLUTION-START
        match *self {
            Shape::Circle { r } => 2.0 * PI * r,
            Shape::Rect { w, h } => 2.0 * (w + h),
            Shape::Triangle { a, b, c } => a + b + c,
        }
        // SOLUTION-END
    }

    /// All dimensions positive, and triangles satisfy the triangle inequality.
    pub fn is_valid(&self) -> bool {
        // SOLUTION-START
        match *self {
            Shape::Circle { r } => r > 0.0,
            Shape::Rect { w, h } => w > 0.0 && h > 0.0,
            Shape::Triangle { a, b, c } => {
                a > 0.0 && b > 0.0 && c > 0.0 && a + b > c && a + c > b && b + c > a
            }
        }
        // SOLUTION-END
    }

    /// Multiply every dimension by `k`, in place. Note `&mut self`.
    pub fn scale(&mut self, k: f64) {
        // SOLUTION-START
        match self {
            Shape::Circle { r } => *r *= k,
            Shape::Rect { w, h } => {
                *w *= k;
                *h *= k;
            }
            Shape::Triangle { a, b, c } => {
                *a *= k;
                *b *= k;
                *c *= k;
            }
        }
        // SOLUTION-END
    }

    /// "circle", "rect" or "triangle".
    pub fn kind(&self) -> &'static str {
        // SOLUTION-START
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rect { .. } => "rect",
            Shape::Triangle { .. } => "triangle",
        }
        // SOLUTION-END
    }
}

/// The shape with the largest area, or `None` for an empty slice.
/// Note the return type borrows from the input — no clone needed.
pub fn largest(shapes: &[Shape]) -> Option<&Shape> {
    // SOLUTION-START
    shapes.iter().max_by(|a, b| a.area().total_cmp(&b.area()))
    // SOLUTION-END
}
