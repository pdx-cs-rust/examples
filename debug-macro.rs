#[allow(unused)]
const DEBUG: bool = true;

#[allow(unused)]
macro_rules! debug {
    ($msg:literal) => {
        if DEBUG {
            eprintln!("{}", $msg as &'static str);
        }
    };
    ($msg:literal, $x:expr) => {
        if DEBUG {
            eprintln!("{}: {:?}", $msg as &'static str, $x);
        }
    };
}

#[allow(unused)]
fn bad_fib(n: u64) -> u64 {
    match n {
        i@(0|1) => i,
        n => bad_fib(n - 1) + bad_fib(n - 2),
    }
}

fn main() {
    debug!("running");
    debug!("running", "yep");
}
