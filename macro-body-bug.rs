macro_rules! square {
    ($n:expr) => { $n:expr * $n:expr };
}

fn main() {
    let mut n = 3;
    println!("3**2={}", square!(n));
}
