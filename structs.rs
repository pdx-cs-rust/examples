/// Examples of structs and enums.
/// Bart Massey 2021

// Struct example.

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {                                                          
    fn x_coord(&self) -> u64 {
        self.x
    }
}

// Tuple struct example.

struct TPoint(u64, u64);

impl TPoint {                                                          
    fn x_coord(&self) -> u64 {
        // or "self.0"
        let TPoint(x, _) = *self;
        x
    }
}                                                                     

// Enum example.

pub enum Color {
    Red,
    Green,
    Blue,
    Custom(f64, f64, f64),
}

impl Color {
    fn red(&self) -> f64 {
        match self {
            Color::Red => 1.0,
            Color::Green => 0.0,
            Color::Blue => 0.0,
            Color::Custom(r, _, _) => *r,
        }
    }
}

fn main() {
    let p = Point { x: 0, y: 0 };
    println!("{:?}", p);
    println!("{}", p.x_coord());

    let p = TPoint(0, 0);
    println!("{}", p.x_coord());

    let c = Color::Custom(0.5, 0.5, 0.0);
    println!("{}", c.red());
}
