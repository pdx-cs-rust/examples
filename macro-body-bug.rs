macro_rules! square {
    ($n:expr) => { $n * $n };
}

fn main() {
    let n = 3;
    println!("3**2={}", square!(n));
}
