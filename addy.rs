use std::ops::Add;

pub trait Numy
where
    Self: Add<Self, Output = Self> + Sized,
{
    type Rendering;

    fn addy(lhs: Self, rhs: Self) -> Self {
        lhs + rhs
    }

    fn render(self) -> Self::Rendering;
}

impl Numy for f64 {
    type Rendering = String;

    fn render(self) -> String {
        self.to_string()
    }
}

impl Numy for u64 {
    type Rendering = Vec<u8>;

    fn render(mut self) -> Vec<u8> {
        let mut result = Vec::new();
        while self > 0 {
            result.push((self % 10) as u8);
            self /= 10;
        }
        result.reverse();
        result
    }
}

fn main() {
    println!("{}", (1.0).render());
    println!("{:?}", u64::addy(100, 12).render())
}
