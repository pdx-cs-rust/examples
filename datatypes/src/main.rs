fn _even1() {
    let x = 5;
    if x % 2 == 1 {
        println!("even {}", x + 1);
    } else {
        let q = '?';
        println!("{}", q);
    }
}

fn _even2() {
    let x = 5;
    let x =
        if x % 2 == 1 {
            x + 1
        } else {
            x
        };
    println!("even {}", x);
}

fn _evens() {
    for i in (0..10).step_by(2) {
        println!("{}", i);
    }
}

fn _one() {
    let x = { 1 };
    println!("{}", x);
}

fn _matchy(x: u8) {
    let y = match x {
        0 => 5,
        1 => panic!("oops"),
        p => p + 1,
    };
    println!("{}", y);
}

fn _diverge() -> ! {
    println!("bye");
    std::process::exit(0);
}

fn _fn_values() {
    fn returns_unit() {
        println!("in function");
    }
    let f = returns_unit;
    let x = f();
    println!("{:?}", x);
}

fn copy() {
    // let x = "string".to_string();
    let x = true;
    let y = x;
    let z = x;
    println!("{} {} {}", x, y, z);
}

fn main() {
    copy();
}
