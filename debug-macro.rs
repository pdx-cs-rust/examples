#[allow(unused)]
const DEBUG: bool = true;

#[allow(unused)]
macro_rules! debug {
    ($msg:literal, $x:expr) => {
        if DEBUG {
            eprintln!("{}: {:?}", $msg as &'static str, $x);
        }
    };
}

fn main() {
    debug!("running", Some(5));
}
