use rc::cell::Counter;

/// Increment the given counter. Note that this function
/// takes `counter` by immutable reference.
fn update_counter(counter: &Counter) {
    counter.incr();
}

fn main() {
    let counter = Counter::default();
    println!("{}", counter.value());
    update_counter(&counter);
    println!("{}", counter.value());
}
