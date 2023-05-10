struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(&self) -> i32 {
        self.x + self.y
    }

    fn euclid(&self) -> i32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}


fn main() {
    let p = Point { x: 17, y: 5 };
    println!("{}", Point::manhattan(&p));
    println!("{}", (&p).manhattan());
    println!("{}", p.manhattan());
}
