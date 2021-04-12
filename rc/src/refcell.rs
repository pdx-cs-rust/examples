//! An ADT for interior-mutable counting up.
//!
//! # Examples
//!
//!     # use rc::refcell::Counter;
//!     let c0 = Counter::default();
//!     c0.incr();
//!     assert_eq!(c0.value(), 1);
//!     let c1 = Counter::clone(&c0);
//!     c0.incr();
//!     c1.incr();
//!     assert_eq!(c0.value(), 3);
//!     assert_eq!(c1.value(), 3);

use crate::simple;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

/// An interior-mutable shared counter over the restricted `simple::Counter`.
#[derive(Debug, Default, Clone)]
pub struct Counter(Rc<RefCell<simple::Counter>>);

impl Counter {
    // Note that this method takes `self` by immutable
    // reference, then changes it anyway.
    /// Increase the count of the contained counter by one.
    pub fn incr(&self) {
        let mut count = self.0.borrow_mut();
        count.incr();
    }

    /// Return the value of the contained counter.
    pub fn value(&self) -> u64 {
        let count = self.0.borrow();
        count.value()
    }

    /// Increase the count and return the new count.
    pub fn incr_value(&self) -> u64 {
        self.incr();
        self.value()
    }

    pub fn count_mut(&self) -> RefMut<simple::Counter> {
        self.0.borrow_mut()
    }
}
