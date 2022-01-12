fn sum_off(c: u32) -> u32 {
    (1u32..=3).map(|n| { n * n + c }).sum::<u32>()
}

fn main() {
    println!("{}", sum_off(7));
}
