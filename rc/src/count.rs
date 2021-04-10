//! An ADT for counting up. Deliberately does not implement most traits.
//!      
//! # Examples
//!
//!     # use rc::count::Count;
//!     let mut count = Count::default();
//!     count.incr();
//!     assert_eq!(count.value(), 1);

/// A non-negative count.
#[derive(Debug, Default)]
pub struct Count(u64);

impl Count {
    /// Increase the count by one.
    pub fn incr(&mut self) {
        self.0 += 1;
    }

    /// Return the current count.
    pub fn value(&self) -> u64 {
        self.0
    }
}
