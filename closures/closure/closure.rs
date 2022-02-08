fn triple(x: u32) -> u32 {
    x + x + x
}

fn foo() {
    let mut y = 2;
    let mut f: Box<dyn FnOnce(u32) -> u32> = Box::new(|x| { y += 1; x * y });
    println!("{}", f(4));
    f = Box::new(|x| x + x);
    println!("{}", f(4));
    f = Box::new(triple);
    println!("{}", f(4));
}

fn main() {
    foo();

    /*
    let mut y = 2;
    let mut powerer = move |x: u64| { y += 1; u64::pow(x, y) };
    println!("{}", powerer(3));
    powerer = move |x: u64| { y += 1; u64::pow(x, y) };
    println!("{}", powerer(3));
    println!("{}", y);
    */
}
