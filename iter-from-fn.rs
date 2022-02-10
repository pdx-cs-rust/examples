fn main() {
    let mut c = 'a';
    let iter = std::iter::from_fn(|| {
        if c >= 'e' {
            return None;
        }
        let result = c;
        c = std::char::from_u32(c as u32 + 1).unwrap();
        Some(result)
    });
    let xs: String = iter.collect();
    println!("{xs}");
}
