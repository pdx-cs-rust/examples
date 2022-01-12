#[derive(Debug, Clone)]
struct P<T>(T, T);

impl<T> P<T> {
    fn first(&self) -> T
        where T: Clone
    {
        self.0.clone()
    }

    fn sum(&self) -> T
        where T: std::ops::Add<Output = T>, T: Clone
    {
        self.0.clone() + self.1.clone()
    }

    fn pair(&self) -> (Self, Self)
        where Self: Clone
    {
        (self.clone(), self.clone())
    }
}

#[derive(Debug, Clone)]
struct Eg;

fn main() {
    let p = P(1u32, 2);
    println!("{}", p.sum());
    let p = P("hello", "world");
    println!("{}", p.first());
    let p = P(Eg, Eg);
    println!("{:?}", p.pair());
}
