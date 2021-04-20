#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point{ x, y }
    }
    
    fn flip(&mut self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }
    
    fn flipped(&self) -> Point {
        Point{ x: self. y, y: self. x }
    }
    
    fn collapse(self) -> i64 {
        self.x + self.y
    }
}

fn main() {
    let mut p = Point::new(3, 4);
    println!("{:?}", p.flipped());
    p.flip();
    println!("{:?}", p);
    println!("{}", p.collapse());
}