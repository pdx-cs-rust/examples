macro_rules! reargument {
    ($x:tt ! ( $($_:tt),* )) => ($x!("huh"))
}

fn main() {
    reargument!(println!("{} {}", x, y));
}
