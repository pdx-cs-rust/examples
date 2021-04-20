// https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html

// A tuple struct that can be compared
#[derive(Debug, PartialEq, PartialOrd)]
struct Centimeters(f64);

// A tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// A vanilla tuple struct
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error! `Seconds` can't be printed, because it doesn't implement the
    // `Debug` trait
    //println!("One second looks like: {}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared, because it doesn't implement the
    // `PartialEq` trait
    //let _this_is_true = _one_second == _one_second;
    // TODO ^ Try uncommenting this line

    let yard = Inches(36);

    println!("One yard === {:?}", yard);

    let meter = Centimeters(100.0);

    let cmp = if yard.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("one yard is {} than one meter", cmp);
}
