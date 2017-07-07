use std::cmp;
use std::fmt;
use std::ops;


#[derive(Debug, Clone, Copy)]
pub struct Degree(f64);

impl Degree {
    pub fn new(value: f64) -> Option<Degree> {
        match value {
            _ if value.is_nan() => None,
            _ if value > 1.0    => None,
            _ if value < 0.0    => None,
            _ => Some(Degree(value)),
        }
    }

    pub fn one() -> Degree {
        Degree(1.0)
    }

    pub fn zero() -> Degree {
        Degree(0.0)
    }

    pub fn and(self, other: Degree) -> Degree {
        Degree(self.0.min(other.0))
    }

    pub fn p_and(self, other: Degree) -> Degree {
        Degree(self.0 * other.0)
    }

    pub fn or(self, other: Degree) -> Degree {
        Degree(self.0.max(other.0))
    }

    pub fn p_or(self, other: Degree) -> Degree {
        Degree((self.0 + other.0) - (self.0 * other.0))
    }

    pub fn not(self) -> Degree {
        Degree(1.0 - self.0)
    }
}


impl cmp::PartialEq for Degree {
    fn eq(&self, other: &Degree) -> bool {
        (self.0 - other.0).abs() < 1.0e-6
    }
}


impl cmp::PartialOrd for Degree {
    fn partial_cmp(&self, other: &Degree) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}


impl ops::Mul for Degree {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Degree(self.0 * rhs.0)
    }
}

impl ops::Sub for Degree {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Option<Self> {
        if self.0 < rhs.0 {
            None
        } else {
            Some(Degree(self.0 - rhs.0))
        }        
    }
}

impl ops::Add for Degree {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Option<Self> {
        if self.0 + rhs.0 > 1.0 {
            None
        } else {
            Some(Degree(self.0 + rhs.0))
        }        
    }
}

impl ops::Div for Degree {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Degree(self.0 / rhs.0)
    }
}


impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



#[cfg(test)]
mod test {
    
    use quickcheck::{Arbitrary, Gen};
    use super::Degree;

    impl Arbitrary for Degree {
        fn arbitrary<G: Gen>(g: &mut G) -> Degree {
            Degree(g.gen_range(0.0, 1.0))
        }
    }


    quickcheck! {
        fn and(x: Degree, y: Degree) -> bool {
            x.and(y) <= x && x.and(y) <= y
        }
    }

    quickcheck! {
        fn p_and(x: Degree, y: Degree) -> bool {
            x.p_and(y) <= x && x.and(y) <= y
        }
    }

    quickcheck! {
        fn or(x: Degree, y: Degree) -> bool {
            x.or(y) >= x && x.or(y) >= y
        }
    }

    quickcheck! {
        fn p_or(x: Degree, y: Degree) -> bool {
            x.p_or(y) >= x && x.p_or(y) >= y
        }
    }

    quickcheck! {
        fn not(x: Degree) -> bool {
            x.not().not() == x            
        }
    }
    
}