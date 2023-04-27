macro_rules! make_point {
    ($name_x:ident, $name_y:ident, $x:expr, $y:expr, $t:ty) =>
        { let $name_x: $t = $x; let $name_y: $t = $y; }
}

fn main() {
    make_point!(x, y, 3, 2, u32);
    println!("{} {}", x, y);
}
