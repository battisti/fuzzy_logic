pub mod option {
 
    pub trait Flatten<T> {
        fn flatten(self) -> Option<T>;
    }

    impl<T> Flatten<T> for Option<Option<T>> {
        fn flatten(self) -> Option<T> {
            match self {
                None => None,
                Some(v) => v,
            }
        }
    }

}


pub mod slice {
    use std::cmp;

    pub fn diff<T: PartialEq + Copy>(xs: &[T], ys: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(xs.len());
        'outer: for x in xs.iter() {
            for y in ys.iter() {
                if x == y {
                    continue 'outer;
                }
            }
            result.push(*x);
        }
        result
    }

    pub fn intersect<T: PartialEq + Copy>(xs: &[T], ys: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(cmp::min(xs.len(), ys.len()));	
        'outer: for x in xs.iter() {
            for y in ys.iter() {
                if x == y {
                    result.push(*x);
                    continue 'outer;
                }
            }
        }
        result
    }

}



#[cfg(test)]
mod test {

    use super::slice::{intersect, diff};

	#[test]
	fn intersection() {
		let xs = [1, 2, 3, 4, 5];
		let ys = [2, 1, 9, 3, 7];
		let zs = intersect(&xs, &ys);

		assert_eq!(vec![1, 2, 3], zs);
	}

	#[test]
	fn difference() {
		let xs = [1, 2, 3, 4, 5];
		let ys = [2, 1, 9, 3, 7];
		let zs = diff(&xs, &ys);

		assert_eq!(vec![4, 5], zs);
	}

}