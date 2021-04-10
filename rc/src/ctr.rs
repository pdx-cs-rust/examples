use std::cell::Cell;

use crate::count::Count;

/// An interior-mutable counter over the restricted `Count`.
#[derive(Default)]
pub struct Ctr(Cell<Count>);

impl Ctr {
    /// Increase the count of the contained counter by one.
    /// Note that this method takes `self` by immutable
    /// reference, then changes it anyway.
    pub fn incr(&self) {
        let mut count = self.0.replace(Count::default());
        count.incr();
        let _ = self.0.replace(count);
    }

    /// Return the value of the contained counter.
    pub fn value(&self) -> u64 {
        let count = self.0.replace(Count::default());
        let value = count.value();
        let _ = self.0.replace(count);
        value
    }
}
