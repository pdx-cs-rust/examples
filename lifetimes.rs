fn call_me() -> &'static u64 {
    let y = 5;
    return &y;
}

fn main() {
    println!("{}", call_me());
}
