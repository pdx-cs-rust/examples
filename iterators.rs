fn main() {
    let mut i = 0..3;
    while let Some(n) = i.next() {
        print!("{} ", n);
    }
    println!();

    let i = 0..3;
    for n in i {
        print!("{} ", n);
    }
    println!();

    let n: u32 = (1..=3).sum();
    println!("{}", n);

    for n in (0..3).rev() {
        print!("{} ", n);
    }
    println!();
}
