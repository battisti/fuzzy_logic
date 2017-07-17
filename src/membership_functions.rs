use std::cmp::PartialOrd;
use std::ops::{Div, Sub};

use degree::Degree;


pub trait Fuzzy<T: PartialOrd> {
    fn membership(&self, value: T) -> Degree;
}



pub struct Boolean<T: PartialOrd>(T);

impl<T: PartialOrd> Boolean<T> {
    pub fn new(x: T) -> Boolean<T> {
        Boolean(x)
    }
}

impl<T: PartialOrd> Fuzzy<T> for Boolean<T> {
    fn membership(&self, value: T) -> Degree {
        if value < self.0 {
            Degree::zero()
        } else {
            Degree::one()
        }
    }
}


pub struct Gradual<T: PartialOrd + Into<f64>>(T, T);

impl<T> Gradual<T> 
where T: PartialOrd + Into<f64>
{
    pub fn new(x0: T, x1: T) -> Gradual<T> {
        assert!(&x0 < &x1);
        Gradual(x0, x1)
    }
}

impl<T> Fuzzy<T> for Gradual<T> 
where T: PartialOrd + Into<f64> + Div<T, Output = T> + Sub<T, Output = T> + Copy
{
    fn membership(&self, value: T) -> Degree {
        match value {
            _ if value <= self.0 => Degree::zero(),
            _ if value >= self.1 => Degree::one(),
            _ => {
                let d = (value / (self.1 - self.0)) - (self.0 / (self.1 - self.0));
                match Degree::new(d.into()) {
                    None    => Degree::zero(),
                    Some(d) => d,
                }
            },
        }
    }
}


pub struct Triangular<T: PartialOrd + Into<f64>>(T, T, T);

impl<T>  Triangular<T>
where T: PartialOrd + Into<f64> {
    pub fn new(x0: T, x1: T, x2: T) -> Triangular<T> {
        assert!(&x0 < &x1 && &x1 < &x2);
        Triangular(x0, x1, x2)
    }
}

impl<T> Fuzzy<T> for Triangular<T>
where T: PartialOrd + Into<f64> + Div<T, Output = T> + Sub<T, Output = T> + Copy
{
    fn membership(&self, value: T) -> Degree {
        match value {
            _ if value <= self.0 => Degree::zero(),
            _ if value >= self.2 => Degree::zero(),
            _ if value >  self.1 => {
                let d = (self.1 / (self.1 - self.0)) - (value / (self.1 - self.0));
                match Degree::new(d.into()) {
                    None    => Degree::zero(),
                    Some(d) => d,
                }
            }
            _ if value < self.1 => {
                let d = (value / (self.1 - self.0)) - (self.0 / (self.1 - self.0));
                match Degree::new(d.into()) {
                    None    => Degree::zero(),
                    Some(d) => d,
                }
            }
            _ => Degree::one(),
        }    
    }
}


pub struct Trapezoid<T: PartialOrd + Into<f64>>(T, T, T, T);

impl<T> Trapezoid<T>
where T: PartialOrd + Into<f64> {
    pub fn new(x0: T, x1: T, x2: T, x3: T) -> Trapezoid<T> {
        assert!(&x0 < &x1 && &x1 < &x2 && &x2 < &x3);
        Trapezoid(x0, x1, x2, x3)
    }
}

impl<T> Fuzzy<T> for Trapezoid<T>
where T: PartialOrd + Into<f64> + Div<T, Output = T> + Sub<T, Output = T> + Copy {
    fn membership(&self, value: T) -> Degree {
        match value {
            _ if value <= self.0 => Degree::zero(),
            _ if value >= self.1 && value <= self.2 => Degree::one(),
            _ if value >  self.0 && value <  self.1 => {
                let d = (value / (self.1 - self.0)) - (self.0 - (self.1 - self.0));
                match Degree::new(d.into()) {
                    None    => Degree::zero(),
                    Some(d) => d,
                }
            },
            _ if value > self.2 && value < self.3 => {
                let d = (self.3 / (self.3 - self.2)) - (value / (self.3 - self.2));
                match Degree::new(d.into()) {
                    None    => Degree::zero(),
                    Some(d) => d,
                }
            },
            _ => Degree::zero(),
        }
    }
}


#[allow(unused_variables)]
// TODO: handle values as well as references
pub fn gaussian<T>(x: &T, mu: &T, std: &T) -> Degree
    where T : PartialOrd + Into<f64>,
          for <'a> T: Div<&'a T, Output = T> + Sub<&'a T, Output = T>,
          for <'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T>
{
    unimplemented!()
}