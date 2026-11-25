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
        // SOLUTION-START
        Money { cents, currency }
        // SOLUTION-END
    }

    pub fn zero(currency: Currency) -> Self {
        // SOLUTION-START
        Money::new(0, currency)
        // SOLUTION-END
    }

    pub fn cents(&self) -> i64 {
        // SOLUTION-START
        self.cents
        // SOLUTION-END
    }

    pub fn currency(&self) -> Currency {
        // SOLUTION-START
        self.currency
        // SOLUTION-END
    }

    /// Adding two different currencies is an error, not a silent bug.
    pub fn checked_add(self, other: Money) -> Result<Money, MoneyError> {
        // SOLUTION-START
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch {
                expected: self.currency,
                found: other.currency,
            });
        }
        Ok(Money::new(self.cents + other.cents, self.currency))
        // SOLUTION-END
    }

    /// Split into `n` parts that add back up exactly; the first parts get the
    /// leftover cents. $10.00 / 3 → [$3.34, $3.33, $3.33]. Panics if n == 0.
    pub fn split(self, n: usize) -> Vec<Money> {
        // SOLUTION-START
        assert!(n > 0, "cannot split into zero parts");
        let n_i = n as i64;
        let base = self.cents / n_i;
        let remainder = self.cents % n_i;
        (0..n_i)
            .map(|i| Money::new(base + if i < remainder { 1 } else { 0 }, self.currency))
            .collect()
        // SOLUTION-END
    }
}

/// "$12.34", "€0.05", "₹100.00", negative as "-$1.50".
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SOLUTION-START
        let symbol = match self.currency {
            Currency::Usd => "$",
            Currency::Eur => "€",
            Currency::Inr => "₹",
        };
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.unsigned_abs();
        write!(f, "{sign}{symbol}{}.{:02}", abs / 100, abs % 100)
        // SOLUTION-END
    }
}

#[derive(Debug)]
pub struct Account {
    id: AccountId,
    balance: Money,
}

impl Account {
    pub fn open(id: AccountId, currency: Currency) -> Self {
        // SOLUTION-START
        Account {
            id,
            balance: Money::zero(currency),
        }
        // SOLUTION-END
    }

    pub fn id(&self) -> AccountId {
        // SOLUTION-START
        self.id
        // SOLUTION-END
    }

    pub fn balance(&self) -> Money {
        // SOLUTION-START
        self.balance
        // SOLUTION-END
    }

    /// Amount must be positive and in the account's currency.
    pub fn deposit(&mut self, amount: Money) -> Result<(), MoneyError> {
        // SOLUTION-START
        if amount.cents <= 0 {
            return Err(MoneyError::NonPositiveAmount);
        }
        self.balance = self.balance.checked_add(amount)?;
        Ok(())
        // SOLUTION-END
    }

    /// Amount must be positive, in the right currency, and not exceed the balance.
    pub fn withdraw(&mut self, amount: Money) -> Result<(), MoneyError> {
        // SOLUTION-START
        if amount.currency != self.balance.currency {
            return Err(MoneyError::CurrencyMismatch {
                expected: self.balance.currency,
                found: amount.currency,
            });
        }
        if amount.cents <= 0 {
            return Err(MoneyError::NonPositiveAmount);
        }
        if amount.cents > self.balance.cents {
            return Err(MoneyError::InsufficientFunds {
                available: self.balance.cents,
                requested: amount.cents,
            });
        }
        self.balance.cents -= amount.cents;
        Ok(())
        // SOLUTION-END
    }
}
