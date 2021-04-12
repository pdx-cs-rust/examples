//! An ADT for counting up. Deliberately does not implement most traits.
//!      
//! # Examples
//!
//!     # use rc::simple::Counter;
//!     let mut count = Counter::default();
//!     count.incr();
//!     assert_eq!(count.value(), 1);

/// A non-negative counter.
#[derive(Debug, Default)]
pub struct Counter(u64);

impl Counter {
    /// Increase the count by one.
    pub fn incr(&mut self) {
        self.0 += 1;
    }

    /// Return the current count.
    pub fn value(&self) -> u64 {
        self.0
    }
}
