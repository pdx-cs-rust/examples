use std::io::Read;
use std::convert::TryFrom;
use std::error::Error;

fn get_ascii_char() -> Result<char, Box<dyn Error>> {
    let mut stdin = std::io::stdin();
    let mut buf = [0u8];
    stdin.read_exact(&mut buf)?;
    let byte = buf[0];
    let ch = char::try_from(byte)?;
    Ok(ch)
}

fn main() {
    match get_ascii_char() {
        Ok(ch) => println!("{}", ch),
        Err(e) => {
            println!("get_ascii_char: {}", e);
            std::process::exit(1);
        }
    }
}
