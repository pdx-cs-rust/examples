use rc::simple::Counter;

/// A "message" struct. More later.
struct Message<'a> {
    /// Text of message.
    note: &'static str,
    /// Number of times the message has been accessed.
    counter: &'a mut Counter,
}

/// Increment the given count. Note that this function takes
/// `count` by mutable reference.
impl <'a> Message<'a> {
    fn update_count(&mut self) {
        println!("{} {}", self.note, self.counter.value());
        self.counter.incr();
    }
}

#[allow(unreachable_code)]
fn main() {
    todo!();
    // This code will fail to compile because of two mutable
    // references to `count`.
    let mut count = Counter::default();
    let mut left = Message { note: "left", counter: &mut count };
    let mut right = Message { note: "right", counter: &mut count };
    left.update_count();
    right.update_count();
}
