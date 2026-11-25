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
        // SOLUTION-START
        self.x.hypot(self.y)
        // SOLUTION-END
    }

    pub fn dot(self, other: Vec2) -> f64 {
        // SOLUTION-START
        self.x * other.x + self.y * other.y
        // SOLUTION-END
    }

    /// Unit vector in the same direction; `None` for the zero vector.
    pub fn normalized(self) -> Option<Vec2> {
        // SOLUTION-START
        let len = self.length();
        (len > 0.0).then(|| self * (1.0 / len))
        // SOLUTION-END
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        // SOLUTION-START
        Vec2::new(self.x + o.x, self.y + o.y)
        // SOLUTION-END
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        // SOLUTION-START
        Vec2::new(self.x - o.x, self.y - o.y)
        // SOLUTION-END
    }
}

/// `v * 2.0`
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        // SOLUTION-START
        Vec2::new(self.x * k, self.y * k)
        // SOLUTION-END
    }
}

/// `2.0 * v` — implementing a std trait for a std type (f64) is allowed
/// because `Vec2` (a type parameter of the impl) is local.
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        // SOLUTION-START
        v * self
        // SOLUTION-END
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        // SOLUTION-START
        Vec2::new(-self.x, -self.y)
        // SOLUTION-END
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        // SOLUTION-START
        self.x += o.x;
        self.y += o.y;
        // SOLUTION-END
    }
}

/// `(1, 2.5)` — use `{}` for each f64.
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SOLUTION-START
        write!(f, "({}, {})", self.x, self.y)
        // SOLUTION-END
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        // SOLUTION-START
        Vec2::new(x, y)
        // SOLUTION-END
    }
}
