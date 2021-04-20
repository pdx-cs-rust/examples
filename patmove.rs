struct Foo {
    tuple: (String, u64),
}

impl Foo {
    fn is_hello(&self) -> bool {
        match self.tuple {
            (ref s, _) => {
                s == "hello"
            }
        }
    }
}

fn main() {
    let t = Foo { tuple: ("hello".to_string(), 1) };
    println!("{}", t.is_hello());
}
