use shuflr::*;
use rand::thread_rng;

fn main() {
    let v: Vec<char> = ('a'..='z').collect();
    let s: String = v
        .shuffled(&mut thread_rng())
        .flat_map(|c| c.to_uppercase())
        .collect();
    println!("{s}");
}
