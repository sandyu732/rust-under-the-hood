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

// SOLUTION-ONLY-START
struct Parser<'t> {
    tokens: &'t [Token],
    pos: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    fn expr(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.term()?;
        while let Some(op @ (Token::Plus | Token::Minus)) = self.peek().cloned() {
            self.pos += 1;
            let right = self.term()?;
            left = match op {
                Token::Plus => Expr::Add(Box::new(left), Box::new(right)),
                _ => Expr::Sub(Box::new(left), Box::new(right)),
            };
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, CalcError> {
        let mut left = self.unary()?;
        while let Some(op @ (Token::Star | Token::Slash)) = self.peek().cloned() {
            self.pos += 1;
            let right = self.unary()?;
            left = match op {
                Token::Star => Expr::Mul(Box::new(left), Box::new(right)),
                _ => Expr::Div(Box::new(left), Box::new(right)),
            };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, CalcError> {
        if self.peek() == Some(&Token::Minus) {
            self.pos += 1;
            return Ok(Expr::Neg(Box::new(self.unary()?)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, CalcError> {
        match self.next().cloned() {
            Some(Token::Num(n)) => Ok(Expr::Num(n)),
            Some(Token::LParen) => {
                let inner = self.expr()?;
                match self.next() {
                    Some(Token::RParen) => Ok(inner),
                    Some(t) => Err(CalcError::UnexpectedToken(t.clone())),
                    None => Err(CalcError::UnexpectedEnd),
                }
            }
            Some(t) => Err(CalcError::UnexpectedToken(t)),
            None => Err(CalcError::UnexpectedEnd),
        }
    }
}
// SOLUTION-ONLY-END

/// Parse ALL the tokens into one expression. Leftover tokens are an
/// `UnexpectedToken` error.
pub fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    // SOLUTION-START
    let mut p = Parser { tokens, pos: 0 };
    let e = p.expr()?;
    match p.peek() {
        None => Ok(e),
        Some(t) => Err(CalcError::UnexpectedToken(t.clone())),
    }
    // SOLUTION-END
}

/// Evaluate recursively. Division by zero is an error, not `inf`.
pub fn eval(e: &Expr) -> Result<f64, CalcError> {
    // SOLUTION-START
    Ok(match e {
        Expr::Num(n) => *n,
        Expr::Neg(x) => -eval(x)?,
        Expr::Add(a, b) => eval(a)? + eval(b)?,
        Expr::Sub(a, b) => eval(a)? - eval(b)?,
        Expr::Mul(a, b) => eval(a)? * eval(b)?,
        Expr::Div(a, b) => {
            let d = eval(b)?;
            if d == 0.0 {
                return Err(CalcError::DivisionByZero);
            }
            eval(a)? / d
        }
    })
    // SOLUTION-END
}

/// tokenize → parse → eval, using `?` all the way.
pub fn calculate(input: &str) -> Result<f64, CalcError> {
    // SOLUTION-START
    eval(&parse(&tokenize(input)?)?)
    // SOLUTION-END
}

/// Fully parenthesised: `(2 * (3 + 4))`, negation as `(-x)`.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SOLUTION-START
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Neg(x) => write!(f, "(-{x})"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Sub(a, b) => write!(f, "({a} - {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Div(a, b) => write!(f, "({a} / {b})"),
        }
        // SOLUTION-END
    }
}
