fn main() {
    let s = "hello".to_string();
    let rs = &s;
    let rrs: &&String = &rs;
    // Prints hello.
    println!("{}", *rrs);

    let s = "hello".to_string();
    // This takes s by reference.
    // Clippy will warn you that you don't need to clone here.
    let cs: String = s.clone();
    // Prints hello.
    println!("{}", cs);

    let s = "hello".to_string();
    let rs = &s;
    // This takes rs by reference, but then dereferences it automatically.
    let cs: String = rs.clone();
    // Prints hello.
    println!("{}", cs);

    let s: &str = "hello";
    // This would clone the &str reference, so it fails to compile.
    // let cs: String = s.clone();
    let cs: String = s.to_owned();
    // Also acceptable above would have been:
    // let cs: String = s.to_string();
    // let cs: String = String::from(s);
    // let cs: String = s.into();
    // Those last two will eat a copy of s, since &str is copy, since
    // references are copy. So you could still do this:
    // println!("{}", s);
    // Prints hello.
    println!("{}", cs);
}
