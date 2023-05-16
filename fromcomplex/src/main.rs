#[derive(Debug)]
struct MyUnit;

#[derive(Debug)]
#[allow(dead_code)]
enum MyOption<T> {
    Some(T),
    None,
}

impl From<()> for MyOption<MyUnit> {
    fn from(_: ()) -> Self {
        MyOption::Some(MyUnit)
    }
}

fn main() {
    println!("{:?}", <MyOption<MyUnit>>::from(()));
    let o: MyOption<MyUnit> = ().into();
    println!("{:?}", o);
}
