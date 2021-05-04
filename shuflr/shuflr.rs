use rand::seq::SliceRandom;
use rand::Rng;

pub struct IterShuffled<'a, T> {
    values: &'a [T],
    indices: Vec<usize>,
}

impl<'a, T> IterShuffled<'a, T> {
    pub fn new<R>(values: &'a [T], rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let nvalues = values.len();
        let mut indices: Vec<usize> = (0..nvalues).collect();
        indices.shuffle(rng);
        IterShuffled { values, indices }
    }
}

impl<'a, T> Iterator for IterShuffled<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.pop().map(|i| &self.values[i])
    }
}

pub trait Shuffled<T> {
    fn shuffled<R>(&self, rng: &mut R) -> IterShuffled<T>
    where
        R: Rng + ?Sized;
}

impl<T> Shuffled<T> for [T] {
    fn shuffled<R>(&self, rng: &mut R) -> IterShuffled<T>
    where
        R: Rng + ?Sized,
    {
        IterShuffled::new(self, rng)
    }
}

#[test]
pub fn test_shuffled() {
    let mut rng = rand::thread_rng();
    let vals = &['a', 'b', 'c'];
    let mut v = Vec::new();
    for &c in vals.shuffled(&mut rng) {
        v.push(c);
    }
    v.sort();
    assert_eq!(&v, vals);
}
