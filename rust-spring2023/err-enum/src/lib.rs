const DIGIT_VALUES: [(char, u64); 7] = [
    ('M', 1000),
    ('D', 500),
    ('C', 100),
    ('L', 50),
    ('X', 10),
    ('V', 5),
    ('I', 1),
];

fn from_roman_digit(digit: char) -> Option<(usize, u64)> {
    for (rank, (digit_name, digit_value)) in DIGIT_VALUES.into_iter().enumerate() {
        if digit == digit_name {
            return Some((rank, digit_value));
        }
    }
    None
}

/// Given a roman number, return its value. Does not
/// permit numbers of the form `IX` where a smaller digit
/// is left of a larger one.
pub fn from_roman(roman: &str) -> u64 {
    let mut digit_values = Vec::with_capacity(roman.len());
    for digit in roman.chars() {
        if let Some(q) = from_roman_digit(digit) {
            digit_values.push(q);
        } else {
            panic!("bad digit");
        }
    }

    for i in 1..digit_values.len() {
        assert!(digit_values[i].0 <= digit_values[i - 1].0);
    }

    let mut total = 0;
    for (_, v) in digit_values {
        total += v;
    }

    total
}
