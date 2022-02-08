struct PowerClosure {
    y: u32,
}

impl PowerClosure {
    fn new(y: u32) -> Self {
        Self { y }
    }

    fn eval(self, x: u64) -> u64 {
        u64::pow(x, self.y)
    }
}

fn main() {
    let y = 2;
    let power = PowerClosure::new(y);
    println!("{}", power.eval(3));
    println!("{}", power.eval(3));
}
