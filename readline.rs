/// Read from stdin with default.
fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    match stdin.read_line(&mut line) {
        Ok(_) => {
            print!("{}", line);
        }
        Err(e) => {
            eprintln!("read_line: {}", e);
            std::process::exit(1);
        }
    }
}
