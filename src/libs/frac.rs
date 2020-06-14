use std::cmp::Ordering;
use fraction::{GenericFraction, Sign};
use fraction::dynaint::DynaInt::S;
use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Frac {
    _frac: GenericFraction<u32>
}

/// Wrapper around GenericFraction to provide complete ordering
impl Frac {
    pub fn new(num: u32, den: u32) -> Self {
        Frac {
            _frac: GenericFraction::new(num, den)
        }
    }

    pub fn numer(&self) -> Option<&u32> {self._frac.numer()}

    pub fn denom(&self) -> Option<&u32> {self._frac.denom()}

    pub fn sign(&self) -> Option<Sign> {self._frac.sign()}

    pub fn infinity() -> Self {
        Self { _frac: GenericFraction::infinity() }
    }

    pub fn neg_infinity() -> Self {
        Self { _frac: GenericFraction::neg_infinity() }
    }

}

// Order Traits
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> Ordering {
        (self._frac.numer().unwrap() * other._frac.denom().unwrap())
            .cmp(&(self._frac.denom().unwrap() * other._frac.numer().unwrap()))
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        (self._frac.numer().unwrap() * other._frac.denom().unwrap())
            .eq(&(self._frac.denom().unwrap() * other._frac.numer().unwrap()))
    }
}

impl Eq for Frac {}

// Arithmetic trait
impl Add for Frac {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { _frac: self._frac + other._frac }
    }
}

impl Sub for Frac {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { _frac: self._frac - other._frac }
    }
}

// From<T> traits
impl From<u16> for Frac {
    fn from(num: u16) -> Self {
        Self { _frac: GenericFraction::from(num) }
    }
}

impl From<u32> for Frac {
    fn from(num: u32) -> Self {
        Self { _frac: GenericFraction::from(num) }
    }
}

impl<N, D> From<(N, D)> for Frac
where N: fmt::Display,
      D: fmt::Display
{
    fn from(pair: (N, D)) -> Self {
        Self { _frac: GenericFraction::from(pair) }
    }
}

impl From<f32> for Frac {
    fn from(f: f32) -> Self {
        Self { _frac: GenericFraction::from(f) }
    }
}

impl From<f64> for Frac {
    fn from(f: f64) -> Self {
        Self { _frac: GenericFraction::from(f) }
    }
}

// Cloning
impl Clone for Frac {
    fn clone(&self) -> Self {
        Self { _frac: self._frac.clone() }
    }
}

// Hash
impl Hash for Frac {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._frac.hash(state);
    }
}

mod tests {
    use super::Frac;
    use std::collections::BTreeMap;

    #[test]
    fn test_1 () {
        let a = Frac::new(30, 4);
        let b = Frac::new(2, 5);
        println!("{:#?}", &a);
        println!("{:#?}", a.clone() - b);
        println!("{:#?}", a.clone() + Frac::from(1u32));
        println!("{:#?}", a + Frac::from(1u32) == Frac::from((17, 2)));
        let m: BTreeMap<Frac, String> = BTreeMap::new();
    }
}
