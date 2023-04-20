macro_rules! reargument {
    ($x:ident ! ( $($_:tt),* )) => ($x!("huh"))
}

fn main() {
    reargument!(println!("{} {}", x, y));
}
