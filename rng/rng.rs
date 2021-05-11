use rand::Rng;

fn main() {
    for _ in 0..10 {
        println!("{}", rand::thread_rng().gen_range(1..=6));
    }
}
