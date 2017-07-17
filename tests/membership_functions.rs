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


mod gradual_membership {
    
    use fuzzy_logic::degree::Degree;
    use fuzzy_logic::membership_functions::{Gradual, Fuzzy};

    #[test]
    fn large_value() {
        let degree = Gradual::new(0.1, 0.5).membership(0.9);
        assert_eq!(degree, Degree::one());
    }
    
    #[test]
    fn boundary() {
        let degree = Gradual::new(0.1, 0.5).membership(0.5);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn small_value() {
        let degree = Gradual::new(0.1, 0.5).membership(0.05);
        assert_eq!(degree, Degree::zero());
    }

}


mod triangular_membership {

    use fuzzy_logic::degree::Degree;
    use fuzzy_logic::membership_functions::{Triangular, Fuzzy};

    #[test]
    fn large_value() {
        let degree = Triangular::new(0.1, 0.5, 0.9).membership(2.5);
        assert_eq!(degree, Degree::zero());
    }
    
    #[test]
    fn boundary() {
        let degree = Triangular::new(0.1, 0.5, 0.9).membership(0.5);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn small_value() {
        let degree = Triangular::new(0.1, 0.5, 0.9).membership(0.05);
        assert_eq!(degree, Degree::zero());
    }

}


mod trapezoid_membership {

    use fuzzy_logic::degree::Degree;
    use fuzzy_logic::membership_functions::{Trapezoid, Fuzzy};

    #[test]
    fn large_value() {
        let degree = Trapezoid::new(0.1, 0.3, 0.6, 0.9).membership(2.5);
        assert_eq!(degree, Degree::zero());
    }
    
    #[test]
    fn boundary() {
        let degree = Trapezoid::new(0.1, 0.3, 0.6, 0.9).membership(0.6);
        assert_eq!(degree, Degree::one());
    }

    #[test]
    fn small_value() {
        let degree = Trapezoid::new(0.1, 0.3, 0.6, 0.9).membership(0.05);
        assert_eq!(degree, Degree::zero());
    }

}


// TODO: tests for other membership functions
mod gaussian_membership {}