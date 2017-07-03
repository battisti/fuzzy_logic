use std::cmp;
use std::fmt;
use std::ops;


#[derive(Debug, Clone, Copy)]
pub struct Degree {
    value: f64
}


impl Degree {
    pub fn new(value: f64) -> Option<Degree> {
        match value {
            _ if value.is_nan() => None,
            _ if value > 1.0    => None,
            _ if value < 0.0    => None,
            _ => Some(Degree { value: value }),
        }
    }

    pub fn one() -> Degree {
        Degree { value: 1.0 }
    }

    pub fn zero() -> Degree {
        Degree { value: 0.0 }
    }

    pub fn and(&self, other: &Degree) -> Degree {
        Degree { value: self.value.min(other.value) }
    }

    pub fn p_and(&self, other: &Degree) -> Degree {
        Degree { value: self.value * other.value }
    }

    pub fn or(&self, other: &Degree) -> Degree {
        Degree { value: self.value.max(other.value) }
    }

    pub fn p_or(&self, other: &Degree) -> Degree {
        Degree { value: (self.value + other.value) - (self.value * other.value) }
    }

    pub fn not(&self) -> Degree {
        Degree { value: 1.0 - self.value }
    }
}


impl cmp::PartialEq for Degree {
    fn eq(&self, other: &Degree) -> bool {
        (self.value - other.value).abs() < 1.0e-6
    }
}


impl cmp::PartialOrd for Degree {
    fn partial_cmp(&self, other: &Degree) -> Option<cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}


impl ops::Mul for Degree {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Degree { value: self.value * rhs.value }
    }
}


impl ops::Div for Degree {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Degree { value: self.value / rhs.value }
    }
}


impl ops::Sub for Degree {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Option<Self> {
        if self.value - rhs.value < 0.0 {
            None
        } else {
            Some(Degree { value: self.value - rhs.value })
        }
    }
}


impl ops::Add for Degree {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Option<Self> {
        if self.value + rhs.value > 1.0 {
            None
        } else {
            Some(Degree { value: self.value - rhs.value })
        }
    }
}


impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}



#[cfg(test)]
mod test {
    use super::Degree;

    // FIXME: constrain x to valid values for Degree
    quickcheck! {
        fn prop(x: f64) -> bool {
            match Degree::new(x) {
                None    => true,
                Some(d) => d <= Degree::one(),
            }
        }
    }
}