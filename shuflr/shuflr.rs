use rand::seq::SliceRandom;
use rand::Rng;

pub struct IterShuffled<'a, T> {
    values: &'a [T],
    indices: Vec<usize>,
}

impl<'a, T> IterShuffled<'a, T> {
    pub fn shuffled<R>(values: &'a [T], rng: &mut R) -> Self
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

pub trait ShuffledExt<T> {
    fn shuffled<R>(&self, rng: &mut R) -> IterShuffled<T>
    where
        R: Rng + ?Sized;
}

impl<T> ShuffledExt<T> for [T] {
    fn shuffled<R>(&self, rng: &mut R) -> IterShuffled<T>
    where
        R: Rng + ?Sized,
    {
        IterShuffled::shuffled(self, rng)
    }
}

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::*;

    #[test]
    pub fn test_shuffled_works() {
        let mut rng = rand::thread_rng();
        let vals = &['a', 'b', 'c'];
        let mut v = Vec::new();
        for &c in vals.shuffled(&mut rng) {
            v.push(c);
        }
        v.sort();
        assert_eq!(&v, vals);
    }

    struct FakeRng(u64);

    impl RngCore for FakeRng {
        fn next_u32(&mut self) -> u32 {
            let result = self.0 as u32;
            self.0 += 1;
            result
        }
        fn next_u64(&mut self) -> u64 {
            let result = self.0;
            self.0 += 1;
            result
        }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            panic!("fill_bytes")
        }
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            panic!("try_fill_bytes")
        }
    }

    #[test]
    pub fn test_shuffled_shuffles() {
        let mut rng = FakeRng(0);
        let vals = &['a', 'b', 'c'];
        let mut v = Vec::new();
        for &c in vals.shuffled(&mut rng) {
            v.push(c);
        }
        assert_eq!(&v, &['a', 'c', 'b']);
    }
}
