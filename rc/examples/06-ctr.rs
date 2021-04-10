use rc::ctr::Ctr;

/// Increment the given counter. Note that this function
/// takes `counter` by immutable reference.
fn update_counter(counter: &Ctr) {
    counter.incr();
}

fn main() {
    let counter = Ctr::default();
    println!("{}", counter.value());
    update_counter(&counter);
    println!("{}", counter.value());
}
