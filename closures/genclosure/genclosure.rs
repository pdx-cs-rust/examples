fn repeat<F: FnMut(u32)->u32>(mut f: F, x: u32) -> u32 {
    let z = f(x);
    f(z)
}

struct Mutation(String);

impl Mutation {
    fn new<F: FnOnce(String)->String>(mutate: F, s: String) -> Self {
        Mutation(mutate(s))
    }
}

fn main() {
    let mut y = 2;
    println!("{}", repeat(|x| {y += 1; x * y}, 3));

    let pal = |s: String|->String {s.chars().rev().collect()};
    let mutation = Mutation::new(
        pal,
        "hello".to_string(),
    );
    println!("{}", mutation.0);
    let mutation = Mutation::new(
        pal,
        "goodbye".to_string(),
    );
    println!("{}", mutation.0);
}
