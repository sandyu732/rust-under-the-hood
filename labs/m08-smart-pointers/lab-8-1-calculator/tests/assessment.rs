use lab_3_3_tokenizer::{Token, tokenize};
use lab_8_1_calculator::*;

#[test]
fn precedence_and_parentheses() {
    assert_eq!(calculate("1 + 2 * 3"), Ok(7.0));
    assert_eq!(calculate("(1 + 2) * 3"), Ok(9.0));
    assert_eq!(calculate("2 * (3 + 4) - -1"), Ok(15.0));
    assert_eq!(
        calculate("10 - 4 - 3"),
        Ok(3.0),
        "subtraction is left-associative"
    );
    assert_eq!(calculate("8 / 4 / 2"), Ok(1.0));
    assert_eq!(calculate("--3"), Ok(3.0));
}

#[test]
fn builds_the_right_tree() {
    let e = parse(&tokenize("1 + 2 * 3").unwrap()).unwrap();
    assert_eq!(e.to_string(), "(1 + (2 * 3))");
    let e = parse(&tokenize("-(4 - 1)").unwrap()).unwrap();
    assert_eq!(e.to_string(), "(-(4 - 1))");
}

#[test]
fn errors() {
    assert_eq!(calculate("1 / 0"), Err(CalcError::DivisionByZero));
    assert_eq!(calculate("1 / (2 - 2)"), Err(CalcError::DivisionByZero));
    assert_eq!(calculate("1 +"), Err(CalcError::UnexpectedEnd));
    assert_eq!(calculate("(1 + 2"), Err(CalcError::UnexpectedEnd));
    assert_eq!(
        calculate("1 2"),
        Err(CalcError::UnexpectedToken(Token::Num(2.0)))
    );
    assert_eq!(
        calculate(")"),
        Err(CalcError::UnexpectedToken(Token::RParen))
    );
    assert!(matches!(calculate("1 + x"), Err(CalcError::Tokenize(_))));
}
