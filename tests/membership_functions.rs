extern crate fuzzy_logic;


mod boolean_membership {

    use fuzzy_logic::degree::Degree;
    use fuzzy_logic::membership_functions::boolean;

    #[test]
    fn large_value() {
        let degree = boolean(&10.2, &0.5);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn boundary() {
        let degree = boolean(&0.75, &0.75);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn small_valer() {
        let degree = boolean(&(-7.2), &0.5);
        assert_eq!(degree, Degree::zero());
    }

}

// TODO: tests for other membership functions
mod gradual_membership {}
mod reverse_gradual_membership {}
mod triangular_membership {}
mod trapezoid_membership {}
mod gaussian_membership {}