extern crate fuzzy_logic;


mod boolean_membership {

    use fuzzy_logic::degree::Degree;
    use fuzzy_logic::membership_functions::{Boolean, Fuzzy};

    #[test]
    fn large_value() {
        let degree = Boolean::new(0.5).membership(7.1);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn boundary() {
        let degree = Boolean::new(0.75).membership(0.75);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn small_value() {
        let degree = Boolean::new(0.5).membership(-3.0);
        assert_eq!(degree, Degree::zero());
    }

}

// TODO: tests for other membership functions
mod gradual_membership {}
mod reverse_gradual_membership {}
mod triangular_membership {}
mod trapezoid_membership {}
mod gaussian_membership {}