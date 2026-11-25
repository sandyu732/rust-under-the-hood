//! Lab 5.5 — Extension traits and blanket implementations.
//!
//! You can add methods to types you don't own (like `str` and `[f64]`) by
//! defining your own trait and implementing it for them.

use std::fmt::Debug;

pub trait Shout {
    /// "hi" → "HI!"
    fn shout(&self) -> String;
}

impl Shout for str {
    fn shout(&self) -> String {
        // SOLUTION-START
        format!("{}!", self.to_uppercase())
        // SOLUTION-END
    }
}

pub trait Mean {
    /// Arithmetic mean, `None` when empty.
    fn mean(&self) -> Option<f64>;
}

impl Mean for [f64] {
    fn mean(&self) -> Option<f64> {
        // SOLUTION-START
        (!self.is_empty()).then(|| self.iter().sum::<f64>() / self.len() as f64)
        // SOLUTION-END
    }
}

pub trait Describe {
    /// `<<{:?}>>` of the value, e.g. `Some(1)` → "<<Some(1)>>".
    fn describe(&self) -> String;
}

/// Blanket impl: EVERY type that is `Debug` gets `describe()`.
impl<T: Debug + ?Sized> Describe for T {
    fn describe(&self) -> String {
        // SOLUTION-START
        format!("<<{self:?}>>")
        // SOLUTION-END
    }
}
