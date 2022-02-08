use std::ops::Add;

pub trait Numy {
    type Rendering;

    fn addy(lhs: Self, rhs: Self) -> Self
    where
        Self: Add<Self, Output = Self> + Sized,
    {
        lhs + rhs
    }

    fn render(&self) -> Self::Rendering;
}

struct U64String(String);

impl std::ops::Add for U64String {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        fn interpret(s: U64String) -> u64 {
            s.0.parse::<u64>().unwrap()
        }
        U64String((interpret(self) + interpret(rhs)).to_string())
    }
}

impl Numy for U64String {
    type Rendering = String;

    fn render(&self) -> Self::Rendering {
        self.0.clone()
    }
}

impl Numy for f64 {
    type Rendering = String;

    fn render(&self) -> Self::Rendering {
        self.to_string()
    }
}

impl Numy for u64 {
    type Rendering = Vec<u8>;

    fn addy(lhs: Self, rhs: Self) -> Self {
        lhs + rhs + 1
    }

    fn render(&self) -> Self::Rendering {
        let mut value = *self;
        let mut result = Vec::new();
        while value > 0 {
            result.push((value % 10) as u8);
            value /= 10;
        }
        result.reverse();
        result
    }
}

fn main() {
    println!("{}", 1.0f64.render());
    println!("{:?}", u64::addy(100, 12).render());
    let sum = U64String::addy(
        U64String("1".to_string()),
        U64String("2".to_string()),
    );
    println!("{}", sum.render());
}
