use std::cmp::Ordering::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sgn {
    Pos,
    Neg,
    Zero,
}

impl From<i8> for Sgn {
    fn from(x: i8) -> Sgn {
        match x.cmp(&0) {
            Less => Sgn::Neg,
            Equal => Sgn::Zero,
            Greater => Sgn::Pos,
        }
    }
}

#[test]
fn test_sgn_i8() {
    let s: Sgn = (-7i8).into();
    assert_eq!(s, Sgn::Neg);
}

#[derive(Debug)]
pub struct SgnConvertError;

impl TryFrom<Sgn> for u8 {
    type Error = SgnConvertError;
    fn try_from(x: Sgn) -> Result<u8, Self::Error> {
        match x {
            Sgn::Neg => Err(SgnConvertError),
            Sgn::Zero => Ok(0),
            Sgn::Pos => Ok(1),
        }
    }
}

#[test]
fn test_u8_sgn() {
    assert_eq!(u8::try_from(Sgn::Pos).unwrap(), 1);
    assert!(u8::try_from(Sgn::Neg).is_err());
}
