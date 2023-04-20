#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Point {
    Point0,
    Point1(u64),
    Point2(u64, u64),
    Point3(u64, u64, u64),
}
use Point::*;

impl Point {
    fn increase(&mut self, v: u64) {
        match *self {
            Point0 => (),
            Point1(ref mut x) => *x += v,
            Point2(ref mut x, ref mut y) => {
                *x += v;
                *y += v;
            }
            Point3(ref mut x, ref mut y, ref mut z) => {
                *x += v;
                *y += v;
                *z += v;
            }
        }
    }

    fn shrink(self) -> Self {
        match self {
            Point1(_) => Point0,
            Point2(x, _) => Point1(x),
            Point3(x, y, _) => Point2(x, y),
            _ => panic!(),
        }
    }
}

fn main() {
    let mut p = Point2(1, 2);
    println!("{:?}", p);
    p.increase(1);
    println!("{:?}", p);
    println!("{:?}", p.shrink());
}
