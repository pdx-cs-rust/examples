#[allow(unused)]
const DEBUG: bool = true;

#[allow(unused)]
macro_rules! debug {
    ($msg:literal, $($x:expr),+) => {
        if DEBUG {
            eprint!("debug: {}: {}:", line!(), $msg);
            $(eprint!(" {:?}", $x);)*
            eprintln!();
        }
    };
    ($msg:literal) => {
        if DEBUG {
            eprintln!("debug: {}: {}", line!(), $msg);
        }
    };
    () => {
        if DEBUG {
            eprintln!("debug: {}", line!());
        }
    };
}

fn main() {
    debug!("running", Some(5), "x");
    debug!("still running");
    debug!();
}
