use rc::simple::Counter;

/// Increment the given count. Note that this function takes
/// `count` by mutable reference.
fn update_count(count: &mut Counter) {
    count.incr();
}

fn main() {
    let mut count = Counter::default();
    println!("{}", count.value());
    update_count(&mut count);
    println!("{}", count.value());
}
