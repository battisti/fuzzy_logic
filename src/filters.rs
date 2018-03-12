use std::collections::HashMap;
use std::hash::Hash;

/// Estimates the probabilites of something belonging to a class given a vector of observations.
pub fn bayesian<T: Eq + Hash + Copy>(observations: &[&HashMap<T, f64>]) -> HashMap<T, f64> {
    if observations.len() == 1 {
        return observations[0].clone();
    }

    let base = observations[0].clone();
    let next = &observations[1..];

    next.iter()
        .fold(base, |acc, n| {
            let mut normalizing_constant = 0.0;
            for (k, v) in n.iter() {
                normalizing_constant += v * acc[k];
            }

            n.iter().map(|(k, prior)| {
                let likelihood = acc[k];
                let posterior = prior * likelihood / normalizing_constant;
                (*k, posterior)
            }).collect()
        })
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::bayesian;

    #[test]
    fn bayes_filter_result_is_a_probability_distribution() {
        let mut os = vec![HashMap::new(); 3];
        for o in os.iter_mut() {
            o.insert(true,  0.40);
            o.insert(false, 0.60);
        }
        let observations: Vec<&HashMap<_,_>> = os.iter().collect();

        let result = bayesian(&observations);
        let sum = result.values().fold(0.0, |acc, n| acc + n);
        assert!((1.0 - sum).abs() < 0.00001);
    }

    #[test]
    fn bayes_filter_identifies_correct_class() {
        let mut os = vec![HashMap::new(); 3];
        for o in os.iter_mut() {
            o.insert(true,  0.40);
            o.insert(false, 0.60);
        }
        let observations: Vec<&HashMap<_,_>> = os.iter().collect();

        let result = bayesian(&observations);
        let (class, _) = result.iter().max_by_key(|&(_, v)| (v * 100.0) as u64).unwrap();
        assert_eq!(class, &false);
    }

    #[test]
    fn bayes_filter_calculates_correct_probability() {
    let mut os = vec![HashMap::new(); 3];
        for o in os.iter_mut() {
            o.insert(true,  0.40);
            o.insert(false, 0.60);
        }
        let observations: Vec<&HashMap<_,_>> = os.iter().collect();

        let result = bayesian(&observations);
        let (_, p) = result.iter().max_by_key(|&(_, v)| (v * 100.0) as u64).unwrap();
        assert!((0.7714 - p).abs() < 0.001);
    }
}