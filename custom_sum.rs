use std::iter::Sum;

struct MyU8(u8);

impl Sum<MyU8> for u16 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = MyU8>
    {
        iter.map(|MyU8(s)| s as u16).sum()
    }
}

fn main() {
    let s = vec![MyU8(1), MyU8(2), MyU8(3)].into_iter().sum::<u16>();
    println!("{}", s);
}
