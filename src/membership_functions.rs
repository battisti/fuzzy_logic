use std::cmp::PartialOrd;
use std::ops::{Div, Sub};

use degree::Degree;


// TODO: handle values as well as references
pub fn boolean<T: PartialOrd>(x: &T, x0: &T) -> Degree {
    if x < x0 {
        Degree::zero()
    } else {
        Degree::one()
    }
}


// TODO: handle values as well as references
pub fn gradual<T>(x: &T, x0: &T, x1: &T) -> Degree
where T : PartialOrd + Into<f64>,
      for <'a> T: Div<&'a T, Output = T> + Sub<&'a T, Output = T>,
      for <'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T>
{
    assert!(x0 < x1);

    match x {
        _ if x <= x0 => Degree::zero(),
        _ if x >= x1 => Degree::one(),
        _ => {
            let d = (x / &(x1 - x0)) - &(x0 / &(x1 - x0));
            match Degree::new(d.into()) {
                None    => Degree::zero(),
                Some(d) => d,
            }
        },
    }
}


// TODO: handle values as well as references
pub fn reverse_gradual<T>(x: &T, x0: &T, x1: &T) -> Degree
    where T : PartialOrd + Into<f64>,
          for <'a> T: Div<&'a T, Output = T> + Sub<&'a T, Output = T>,
          for <'a, 'b> &'a T: Div<&'b T, Output = T> + Sub<&'b T, Output = T>
{
    assert!(x0 < x1);
    match Degree::one() - gradual(x, x0, x1) {
        None    => Degree::zero(),
        Some(d) => d,
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