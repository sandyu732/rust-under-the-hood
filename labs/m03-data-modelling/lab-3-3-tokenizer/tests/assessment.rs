use lab_3_3_tokenizer::Token::*;
use lab_3_3_tokenizer::*;

#[test]
fn simple_expression() {
    assert_eq!(
        tokenize("3 + 4.5 * (2 - 1)"),
        Ok(vec![
            Num(3.0),
            Plus,
            Num(4.5),
            Star,
            LParen,
            Num(2.0),
            Minus,
            Num(1.0),
            RParen
        ])
    );
}

#[test]
fn no_whitespace_and_division() {
    assert_eq!(tokenize("10/2"), Ok(vec![Num(10.0), Slash, Num(2.0)]));
    assert_eq!(tokenize("  "), Ok(vec![]));
}

#[test]
fn unexpected_character_reports_position() {
    assert_eq!(
        tokenize("1 + x"),
        Err(TokenizeError::UnexpectedChar { ch: 'x', pos: 4 })
    );
    assert_eq!(
        tokenize("2 ^ 3"),
        Err(TokenizeError::UnexpectedChar { ch: '^', pos: 2 })
    );
}

#[test]
fn bad_numbers() {
    assert_eq!(
        tokenize("1 + 1.2.3"),
        Err(TokenizeError::BadNumber {
            text: "1.2.3".into(),
            pos: 4
        })
    );
}
