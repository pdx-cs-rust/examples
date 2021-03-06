macro_rules! nargs {
    () => { 0 };
    ($_:expr $(, $rest:expr)* $(,)?) => { nargs!($($rest),*) + 1 };
}

fn main() {
    println!("{}", nargs!(
        "a",
        'b',
        5,
    ));
}
