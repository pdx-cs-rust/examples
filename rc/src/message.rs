//! An ADT for "messages", logging the number of times the
//! message text has been "accessed".
//!
//! # Examples
//!
//!     # use rc::message::Message;
//!     let hello = Message::new("hello");
//!     assert_eq!(hello.text(), "hello");
//!     assert_eq!(hello.accesses(), 1);
//!     let htmp = hello.clone();
//!     assert_eq!(htmp.text(), "hello");
//!     assert_eq!(hello.accesses(), 2);

use crate::counter::Counter;

/// Message to print plus usage counter.
#[derive(Clone)]
pub struct Message {
    /// Message text.
    message: &'static str,
    /// Number of times the message text has been accessed.
    counter: Counter,
}

impl Message {

    /// Make a new message.
    pub fn new(message: &'static str) -> Message
    {
        Message { message, counter: Counter::default() }
    }

    /// Access the message text.
    pub fn text(&self) -> &'static str {
        self.counter.incr();
        self.message
    }

    /// Report number of accesses.
    pub fn accesses(&self) -> u64 {
        self.counter.value()
    }
}
