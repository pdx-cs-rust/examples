fn f((x, y): (u32, u32)) {
    println!("{}", x);
    println!("{}", y);
}

fn main() {
    let mut x = (3, 5);
    f(x);
    if let (3, y) = x {
        println!("{}", y);
    }
    while let (3, y) = x {
        println!("{}", y);
        x.0 += 1;
    }
}
