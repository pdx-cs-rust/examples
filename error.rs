use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum PointError {
    CoordDescErr(std::num::ParseIntError),
    CoordCountErr(usize),
}

impl std::fmt::Display for PointError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PointError::CoordDescErr(ref e) => write!(f, "coord parse error: {}", e),
            PointError::CoordCountErr(n) => write!(f, "expected 2 coords, got {}", n),
        }
    }
}

impl std::error::Error for PointError {}

impl FromStr for Point {
    type Err = PointError;

    fn from_str(s: &str) -> Result<Self, PointError> {
        let coords = s
            // XXX This will accept ((1,2)) etc.
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|s| s.trim().parse().map_err(PointError::CoordDescErr))
            .collect::<Result<Vec<i32>, PointError>>()?;

        if coords.len() != 2 {
            return Err(PointError::CoordCountErr(coords.len()));
        }

        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = Point::from_str(
        &std::env::args()
            .nth(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "missing program argument"))?,
    )?;
    println!("{:?}", p);
    Ok(())
}
