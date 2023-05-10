fn read_bytes<R: std::io::Read>(r: &mut R, n: usize) ->
    Result<Vec<u8>, std::io::Error>
{
    let mut result = Vec::with_capacity(n);
    result.resize(n, 0);
    let mut target: &mut [u8] = &mut result[..];
    loop {
        let nread = r.read(target)?;
        if nread == 0 {
            result.resize(target.len(), 0);
            return Ok(result);
        }
        target = &mut target[nread..];
        if target.len() == 0 {
            return Ok(result);
        }
    }
}

fn print_string<W: std::io::Write>(w: &mut W, s: &str) {
    w.write_all(s.as_bytes()).unwrap();
    w.write(b"\n").unwrap();
}

fn main() {
    println!("hello world");
    let mut stdout = std::io::stdout();
    print_string(&mut stdout, "print string!");
}
