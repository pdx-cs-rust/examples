#[allow(unused)]
const DEBUG: bool = true;

#[allow(unused)]
macro_rules! debug {
    ($msg:literal, $($x:expr),+) => {
        if DEBUG {
            eprint!("debug: {}:", $msg);
            $(eprint!(" {:?}", $x);)*
            eprintln!();
        }
    };
    ($msg:literal) => {
        if DEBUG {
            eprintln!("debug: {}", $msg);
        }
    };
    () => {
        if DEBUG {
            eprintln!("debug");
        }
    };
}

fn main() {
    debug!("running", Some(5), "x");
    debug!("still running");
    debug!();
}
