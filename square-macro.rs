macro_rules! square {
    ($s:expr) => {{
        let n = $s;
        n * n
    }};
}

fn main() {
    let mut n = 3;
    println!("3**2={}", square!(n));
    println!("4**2={}", square!({n += 1; n}));
}
