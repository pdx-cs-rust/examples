mod fancymod;
mod reallyfancymod;

use reallyfancymod::subfn;

mod mymod {
    pub fn hello() {
        println!("hello world");
    }
}

#[cfg(test)]
mod tests {
    const N: u64 = 5;

    #[test]
    fn my_test() {
        assert!(N % 2 == 1);
    }
}

fn main() {
    use mymod::*;
    hello();
    fancymod::fancy_hello();
    subfn::really_fancy_hello("me");
}
