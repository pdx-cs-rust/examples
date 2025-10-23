macro_rules! square {
    ($s:expr) => {
        ($s) * ($s)
    };
}

fn main() {
    let mut n = 3_u8;
    println!("3**2={}", square!(n));
    println!("4**2={}", square!({n += 1; n}));
    println!("{}", n);
}
