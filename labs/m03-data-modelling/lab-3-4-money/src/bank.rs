#![allow(unused)] // TODO: delete this line once the lab is done
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Usd,
    Eur,
    Inr,
}

/// Newtype: an account ID can't be mixed up with any other u64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(pub u64);

/// An amount in minor units (cents). Fields are private on purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    cents: i64,
    currency: Currency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    CurrencyMismatch { expected: Currency, found: Currency },
    NonPositiveAmount,
    InsufficientFunds { available: i64, requested: i64 },
}

impl Money {
    pub fn new(cents: i64, currency: Currency) -> Self {
        todo!("lab-3-4-money: implement me")
    }

    pub fn zero(currency: Currency) -> Self {
        todo!("lab-3-4-money: implement me")
    }

    pub fn cents(&self) -> i64 {
        todo!("lab-3-4-money: implement me")
    }

    pub fn currency(&self) -> Currency {
        todo!("lab-3-4-money: implement me")
    }

    /// Adding two different currencies is an error, not a silent bug.
    pub fn checked_add(self, other: Money) -> Result<Money, MoneyError> {
        todo!("lab-3-4-money: implement me")
    }

    /// Split into `n` parts that add back up exactly; the first parts get the
    /// leftover cents. $10.00 / 3 → [$3.34, $3.33, $3.33]. Panics if n == 0.
    pub fn split(self, n: usize) -> Vec<Money> {
        todo!("lab-3-4-money: implement me")
    }
}

/// "$12.34", "€0.05", "₹100.00", negative as "-$1.50".
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("lab-3-4-money: implement me")
    }
}

#[derive(Debug)]
pub struct Account {
    id: AccountId,
    balance: Money,
}

impl Account {
    pub fn open(id: AccountId, currency: Currency) -> Self {
        todo!("lab-3-4-money: implement me")
    }

    pub fn id(&self) -> AccountId {
        todo!("lab-3-4-money: implement me")
    }

    pub fn balance(&self) -> Money {
        todo!("lab-3-4-money: implement me")
    }

    /// Amount must be positive and in the account's currency.
    pub fn deposit(&mut self, amount: Money) -> Result<(), MoneyError> {
        todo!("lab-3-4-money: implement me")
    }

    /// Amount must be positive, in the right currency, and not exceed the balance.
    pub fn withdraw(&mut self, amount: Money) -> Result<(), MoneyError> {
        todo!("lab-3-4-money: implement me")
    }
}
