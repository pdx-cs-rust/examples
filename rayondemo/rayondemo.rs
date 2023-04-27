use rayon::prelude::*;

fn main() {
    let v: Vec<f64> = (0u64..100_000_000).map(|i| i as f64).collect();
    let s: f64 = v.par_iter().sum();
    println!("{}", s);
}
