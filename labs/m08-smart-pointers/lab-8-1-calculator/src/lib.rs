#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 8.1 — Calculator: a recursive `Expr` tree built with `Box`.
//!
//! Uses your Lab 3.3 tokenizer (finish that first). Grammar, lowest to highest
//! precedence — one function per rule is the classic recursive-descent shape:
//!
//! ```text
//! expr    := term (('+' | '-') term)*
//! term    := unary (('*' | '/') unary)*
//! unary   := '-' unary | primary
//! primary := NUMBER | '(' expr ')'
//! ```
//!
//! `cargo run -p lab-8-1-calculator` starts a REPL.

use lab_3_3_tokenizer::{Token, TokenizeError, tokenize};
use std::fmt;

/// A type can't contain itself by value (infinite size), so children are boxed.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("tokenize error: {0:?}")]
    Tokenize(TokenizeError),
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("unexpected token {0:?}")]
    UnexpectedToken(Token),
    #[error("division by zero")]
    DivisionByZero,
}

/// TokenizeError doesn't implement std::error::Error, so thiserror's
/// `#[from]` can't be used; a manual `From` makes `?` convert it.
impl From<TokenizeError> for CalcError {
    fn from(e: TokenizeError) -> Self {
        CalcError::Tokenize(e)
    }
}

/// Parse ALL the tokens into one expression. Leftover tokens are an
/// `UnexpectedToken` error.
pub fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    todo!("lab-8-1-calculator: implement me")
}

/// Evaluate recursively. Division by zero is an error, not `inf`.
pub fn eval(e: &Expr) -> Result<f64, CalcError> {
    todo!("lab-8-1-calculator: implement me")
}

/// tokenize → parse → eval, using `?` all the way.
pub fn calculate(input: &str) -> Result<f64, CalcError> {
    todo!("lab-8-1-calculator: implement me")
}

/// Fully parenthesised: `(2 * (3 + 4))`, negation as `(-x)`.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("lab-8-1-calculator: implement me")
    }
}
