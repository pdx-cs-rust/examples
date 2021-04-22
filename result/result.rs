use std::io::Read;

#[cfg(feature = "ownerror")]
mod my_result {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> MyResult<T, E> {
        fn new(x: Result<T, E>) -> Self {
            match x {
                Ok(x) => MyResult::Ok(x),
                Err(x) => MyResult::Err(x),
            }
        }

        fn unwrap(self) -> T
            where E: std::error::Error
        {
            match self {
                MyResult::Ok(v) => v,
                MyResult::Err(e) => panic!("unwrap failed: {}", e),
            }
        }
    }
}

fn read_file_as_string<P>(filename: P) -> Result<(usize, String), Box<dyn std::error::Error>>
    where P: AsRef<std::path::Path>
{
    let mut f = std::fs::File::open(filename.as_ref())?;
    let mut bytes: Vec<u8> = Vec::with_capacity(1024);
    let n = f.read_to_end(&mut bytes)?;
    let s = std::str::from_utf8(&bytes)?;
    Ok((n, s.to_string()))
}

fn main() {
    match read_file_as_string("/tmp/file") {
        Ok((n, s)) => {
            println!("{}", n);
            print!("{}", s);
        }
        Err(e) => {
            println!("could not read /tmp/file: {}", e);
            std::process::exit(1);
        }
    }
}
