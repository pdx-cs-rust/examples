use rc::count::Count;

/// Increment the given count. Note that this function takes
/// `count` by mutable reference.
fn update_count(count: &mut Count) {
    count.incr();
}

fn main() {
    let mut count = Count::default();
    println!("{}", count.value());
    update_count(&mut count);
    println!("{}", count.value());
}
