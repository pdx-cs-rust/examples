fn main() {
    let mut v: Vec<&'static str> = Vec::new();
    v.push("hello");
    v.push("world");
    println!("{:?}", v);
    println!("{:?}", &v);
    let w = vec!["hello", "world"];
    assert_eq!(v, w);

    let a: [&'static str; 2] = ["hello", "world"];
    println!("{:?}", a);
    let b: [&'static str; 2] = ["hello", "world"];
    assert_eq!(a, b);

    let s: &[&'static str] = &v;
    let t: &[&'static str] = &a;

    assert_eq!(s, t);
    assert_eq!(s.len(), t.len());
}
