#[derive(Debug)]
#[cfg_attr(not(feature = "impldefault"), derive(Default))]
#[allow(dead_code)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "impldefault")]
impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

fn main() {
    let pa: [Point; 3] = Default::default();
    println!("{:?}", pa);

    let pa: [Point; 3] = std::array::from_fn(|_| Point::new(0, 0));
    println!("{:?}", pa);
}
