macro_rules! count_args {
    () => {
        0
    };
    ($arg:tt) => {
        1
    };
    ($arg:tt $($args:tt)*) => {
        1 + count_args!($($args)*)
    };
}


macro_rules! counted_array {
    (let $name:ident : $ty:ty = [ $($values:expr),* ]) => {
        let $name : [ $ty ; count_args!( $($values)* )] = [ $($values),* ];
    };
}

fn main() {
    counted_array!(let a: u32 = [ 1, 2, 3 ]);
    println!("{:?}", a);
}
