fn f<'a>(x: &'a mut u32, y: &'a mut u32) -> &'static u32 {
    *y = 7;
    *x = 8;
    &3
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    let xp = f(&mut x, &mut y);
    println!("*xp={}", *xp);
    println!("x={} y={}", x, y);
}
