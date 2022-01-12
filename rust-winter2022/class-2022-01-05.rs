fn main() {
    let v: Vec<u8> = vec![1, 2];
    let mut it = v.into_iter().rev();
    while let Some(val) = it.next() {
        println!("{}", val);
    }
}
