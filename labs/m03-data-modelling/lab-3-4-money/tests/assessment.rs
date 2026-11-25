use lab_3_4_money::bank::*;

fn dollars(c: i64) -> Money {
    Money::new(c, Currency::Usd)
}

#[test]
fn display() {
    assert_eq!(dollars(1234).to_string(), "$12.34");
    assert_eq!(Money::new(5, Currency::Eur).to_string(), "€0.05");
    assert_eq!(Money::new(10000, Currency::Inr).to_string(), "₹100.00");
    assert_eq!(dollars(-150).to_string(), "-$1.50");
}

#[test]
fn add_same_and_mismatched_currency() {
    assert_eq!(dollars(100).checked_add(dollars(250)), Ok(dollars(350)));
    assert_eq!(
        dollars(100).checked_add(Money::new(1, Currency::Eur)),
        Err(MoneyError::CurrencyMismatch {
            expected: Currency::Usd,
            found: Currency::Eur
        })
    );
}

#[test]
fn split_is_exact() {
    let parts = dollars(1000).split(3);
    assert_eq!(parts, vec![dollars(334), dollars(333), dollars(333)]);
    let total: i64 = dollars(1001).split(7).iter().map(Money::cents).sum();
    assert_eq!(total, 1001);
}

#[test]
fn account_rules() {
    let mut acct = Account::open(AccountId(7), Currency::Usd);
    assert_eq!(acct.id(), AccountId(7));
    assert_eq!(acct.balance(), Money::zero(Currency::Usd));
    acct.deposit(dollars(500)).unwrap();
    assert_eq!(acct.deposit(dollars(0)), Err(MoneyError::NonPositiveAmount));
    assert!(matches!(
        acct.deposit(Money::new(1, Currency::Inr)),
        Err(MoneyError::CurrencyMismatch { .. })
    ));
    assert_eq!(
        acct.withdraw(dollars(600)),
        Err(MoneyError::InsufficientFunds {
            available: 500,
            requested: 600
        })
    );
    acct.withdraw(dollars(200)).unwrap();
    assert_eq!(acct.balance(), dollars(300));
}
