use std::cell::Cell;

use crate::simple;

/// An interior-mutable counter over the restricted `Count`.
#[derive(Default)]
pub struct Counter(Cell<simple::Counter>);

impl Counter {
    /// Increase the count of the contained counter by one.
    /// Note that this method takes `self` by immutable
    /// reference, then changes it anyway.
    pub fn incr(&self) {
        let mut count = self.0.replace(simple::Counter::default());
        count.incr();
        let _ = self.0.replace(count);
    }

    /// Return the value of the contained counter.
    pub fn value(&self) -> u64 {
        let count = self.0.replace(simple::Counter::default());
        let value = count.value();
        let _ = self.0.replace(count);
        value
    }
}
