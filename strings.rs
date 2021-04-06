fn main() {
    let mut s = String::new();
    s.push('a');
    s.push('b');
    println!("{}", s);

    let t = 5u32.to_string();
    println!("{}", t);
    let t = "ab".to_string();
    assert_eq!(s, t);

    let u = format!("{}{}", 'a', 'b');
    assert_eq!(s, u);

    /*
    for i in 0..s.len() {
        println!("{}", s[i]);
    }
    */

    for c in s.chars() {
        println!("{}", c);
    }

    let w: &str = "ab";
    assert_eq!(s, w);

    let w: &str = &s;
    println!("{}", w);
}
