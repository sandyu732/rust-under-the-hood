//! Lab 3.3 — Tokenizer. (Module 8's calculator builds on this crate — keep it!)
//!
//! `"3 + 4.5 * (2 - 1)"` → `[Num(3.0), Plus, Num(4.5), Star, LParen, Num(2.0), Minus, Num(1.0), RParen]`
//!
//! Hint: `let mut chars = input.char_indices().peekable();` then
//! `while let Some((pos, ch)) = chars.next() { match ch { ... } }`.
//! For numbers, keep consuming while `chars.peek()` is a digit or '.'.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeError {
    /// A character that isn't part of any token. `pos` is its byte index.
    UnexpectedChar { ch: char, pos: usize },
    /// Something like "1.2.3". `pos` is the byte index where it starts.
    BadNumber { text: String, pos: usize },
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    // SOLUTION-START
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((pos, ch)) = chars.next() {
        let token = match ch {
            c if c.is_whitespace() => continue,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            c if c.is_ascii_digit() || c == '.' => {
                let mut end = pos + c.len_utf8();
                while let Some(&(i, next)) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        end = i + next.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                let text = &input[pos..end];
                match text.parse::<f64>() {
                    Ok(n) => Token::Num(n),
                    Err(_) => {
                        return Err(TokenizeError::BadNumber {
                            text: text.to_string(),
                            pos,
                        });
                    }
                }
            }
            other => return Err(TokenizeError::UnexpectedChar { ch: other, pos }),
        };
        tokens.push(token);
    }
    Ok(tokens)
    // SOLUTION-END
}
