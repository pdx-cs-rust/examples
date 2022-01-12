#[derive(Debug, Clone)]
struct Point2D(i64, i64);

impl Point2D {
    fn offset_point2d(&mut self, (x_off, y_off): (i64, i64)) {
        self.0 += x_off;
        self.1 += y_off;
    }
}

fn main() {
    let origin = Point2D(0, 0);
    let mut my_point = origin.clone();
    my_point.offset_point2d((1, 1));
    println!("origin = {:?}; mypoint = {:?}", origin, my_point);
}
