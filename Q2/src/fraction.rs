use std::fmt;
pub mod conversions;
pub mod ops;

#[derive(Clone, Copy, Debug)]
pub struct Frac {
    numerator: i64,
    denominator: i64,
}

impl Frac {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Frac {
            numerator,
            denominator,
        }
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
