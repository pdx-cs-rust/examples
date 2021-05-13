fn main() {
    let s = "🦀hello";
    println!("nbytes {}", s.len());
    println!("nchar {}", s.chars().count());
    println!("{}", s.chars().nth(0).unwrap());
    let t: String = s.chars().take(5).collect();
    println!("{}", t);
    let u = s.as_bytes();
    println!("{:02x?}", u);
    let v = std::str::from_utf8(u).unwrap();
    println!("{}", v);
    let v = unsafe { std::str::from_utf8_unchecked(u) };
    println!("{}", v);
    let mut w = u.to_vec();
    w[0] = 0;
    println!("{}", unsafe { std::str::from_utf8_unchecked(&w) });
}
