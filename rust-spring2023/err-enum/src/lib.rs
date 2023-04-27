use thiserror::Error;

#[derive(Debug, Error)]
pub enum RomanError {
    #[error("illegal roman digit {0}")]
    BadDigit(char),
    #[error("roman digit out of order at position {0}")]
    OutOfOrder(usize),
}

const DIGIT_VALUES: [(char, u64); 7] = [
    ('M', 1000),
    ('D', 500),
    ('C', 100),
    ('L', 50),
    ('X', 10),
    ('V', 5),
    ('I', 1),
];

struct RankedDigit {
    rank: usize,
    digit_value: u64,
}

fn from_roman_digit(digit: char) -> Option<RankedDigit> {
    for (rank, (digit_name, digit_value)) in DIGIT_VALUES.into_iter().enumerate() {
        if digit == digit_name {
            return Some(RankedDigit { rank, digit_value });
        }
    }
    None
}

/// Given a roman number, return its value. Does not
/// permit numbers of the form `IX` where a smaller digit
/// is left of a larger one.
pub fn from_roman(roman: &str) -> Result<u64, RomanError> {
    let mut digit_values = Vec::with_capacity(roman.len());
    for digit in roman.chars() {
        if let Some(q) = from_roman_digit(digit) {
            digit_values.push(q);
        } else {
            return Err(RomanError::BadDigit(digit));
        }
    }

    for i in 1..digit_values.len() {
        if digit_values[i].rank < digit_values[i - 1].rank {
            return Err(RomanError::OutOfOrder(i));
        }
    }

    let mut total = 0;
    for rv in digit_values {
        total += rv.digit_value;
    }

    Ok(total)
}

#[test]
fn test_roman() {
    assert_eq!(1001, from_roman("MI").unwrap());
}
