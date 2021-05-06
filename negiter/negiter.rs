pub struct IterNegate<I, T>
where
    I: Iterator<Item = T>,
    T: std::ops::Neg<Output = T>,
{
    iter: I,
}

impl<I, T> Iterator for IterNegate<I, T>
where
    I: Iterator<Item = T>,
    T: std::ops::Neg<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| -v)
    }
}

impl<I, T> IterNegate<I, T>
where
    I: Iterator<Item = T>,
    T: std::ops::Neg<Output = T>,
{
    pub fn negated(iter: I) -> Self {
        Self { iter }
    }
}

pub trait IterNegateExt<I, T>
where
    I: Iterator<Item = T>,
    T: std::ops::Neg<Output = T>,
{
    fn negated(self) -> IterNegate<I, T>;
}

impl<I, T> IterNegateExt<I, T> for I
where
    I: Iterator<Item = T>,
    T: std::ops::Neg<Output = T>,
{
    fn negated(self) -> IterNegate<I, T> {
        IterNegate::negated(self)
    }
}

#[test]
fn test_negated_example() {
    let numbers: Vec<i32> = (0..=5).collect();
    let negs: Vec<i32> = numbers.iter().cloned().negated().collect();
    assert_eq!(numbers.len(), negs.len());
    for (i, &v) in numbers.iter().enumerate() {
        assert_eq!(-v, negs[i]);
    }
}
