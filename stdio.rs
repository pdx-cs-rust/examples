/// Use `Write` so that `writeln!()` and `write!()` work.
/// Use `BufRead` so that `stdin.read_line()` works.
use std::io::{self, Write, BufRead};

fn main() {
    // Get a handle to stdout. Note that we are not bothering to lock it, because we don't care
    // about performance and there's no `BufWrite` trait.
    let mut stdout = io::stdout();
    // This could just as easily have been `println!()`.
    writeln!(stdout, "Trying some stuff").unwrap();
    // Print a prompt and leave the cursor there.
    write!(stdout, "> ").unwrap();
    // Without this the prompt won't appear until way late.
    stdout.flush().unwrap();
    // Get a handle to stdin. Note that this must be a separate step from locking, because the
    // locking borrows the handle.
    let stdin = io::stdin();
    // Get a locked `StdinLocked` guard. This obeys `BufRead`.
    let mut stdin = stdin.lock();
    // Make a placeholder to read a line of text into.
    let mut msg = String::new();
    // Read a line of text as a side effect in `msg`.
    let _ = stdin.read_line(&mut msg).unwrap();
    // Show the line, trimming off the line ending (and any whitespace).
    eprintln!("Got {}", msg.trim_end());
}
