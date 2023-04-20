#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OurOption<T> {
    Some(T),
    None,
}
use OurOption::*;

impl<T> OurOption<T> {
    fn unwrap(self) -> T {
        match self {
            Some(v) => v,
            None => panic!("unwrap of None"),
        }
    }
}

fn true_min(x: u32, y: u32) -> OurOption<u32> {
    if x == y {
        None
    } else {
        Some(u32::min(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OurResult<T, E> {
    Ok(T),
    Err(E),
}
use OurResult::*;

impl<T, E: std::fmt::Debug> OurResult<T, E> {
    fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => panic!("unwrap of Err: {:?}", e),
        }
    }
}

fn divide(dividend: u32, divisor: u32) -> OurResult<u32, String> {
    if divisor == 0 {
        Err(format!("division of {} by zero", dividend))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let q = true_min(4, 5);
    println!("{}", q.unwrap());

    println!("{}", divide(5, 0).unwrap());
}
