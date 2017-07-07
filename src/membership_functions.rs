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
where T: PartialOrd + Into<f64> + Copy
{
    pub fn new(x0: T, x1: T) -> Gradual<T> {
        assert!(x0 < x1);
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



// TODO: handle values as well as references
pub fn triangular<T>(x: &T, x0: &T, x1: &T, x2: &T) -> Degree
where T : PartialOrd + Into<f64>,
      for <'a> T: Div<&'a T, Output = T> + Sub<&'a T, Output = T>,
      for <'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T>
{
    assert!(x0 < x1);
    assert!(x1 < x2);

    match x {
        _ if x <= x0 => Degree::zero(),
        _ if x >= x2 => Degree::zero(),
        _ if x > x1 => {
            let d = (x1 / &(x1 - x0)) - &(x / &(x1 - x0));
            match Degree::new(d.into()) {
                None    => Degree::zero(),
                Some(d) => d,
            }
        }
        _ if x < x1 => {
            let d = (x / &(x1 - x0)) - &(x0 / &(x1 - x0));
            match Degree::new(d.into()) {
                None    => Degree::zero(),
                Some(d) => d,
            }
        }
        _ => Degree::one(),
    }
}


// TODO: handle values as well as references
pub fn trapezoid<T>(x: &T, x0: &T, x1: &T, x2: &T, x3: &T) -> Degree
    where T : PartialOrd + Into<f64>,
          for <'a> T: Div<&'a T, Output = T> + Sub<&'a T, Output = T>,
          for <'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T>
{
    assert!(x0 < x1);
    assert!(x1 < x2);
    assert!(x2 < x3);

    match x {
        _ if x  <= x0         => Degree::zero(),
        _ if x0 < x && x < x1 => {
            let d = (x / &(x1 - x0)) - &(x0 - &(x1 - x0));
            match Degree::new(d.into()) {
                None    => Degree::zero(),
                Some(d) => d,
            }
        }
        _ if x2 < x && x < x3 => {
            let d = (x3 / &(x3 - x2)) - &(x / &(x3 - x2));
            match Degree::new(d.into()) {
                None    => Degree::zero(),
                Some(d) => d,
            }
        },
        _ => Degree::one(),
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