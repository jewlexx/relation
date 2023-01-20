use num_traits::Signed;

/// A structural representation of a relation
///
/// Takes a relation function and a starter number.
/// Then it can calculate the next value based on the current value
///
/// The concept is based on the Mathematic Recursion and Financial Modelling concept of relations
#[derive(Debug, Copy, Clone)]
pub struct Relation<I, F> {
    current_number: I,
    relation: F,
}

impl<I: Signed + Copy, F: FnOnce(I) -> I + Copy> Relation<I, F> {
    /// Create a new [`Relation`]
    pub fn new(starter: I, relation: F) -> Self {
        Self {
            current_number: starter,
            relation,
        }
    }

    /// Calculates and returns the next number in the sequence
    pub fn calculate_next(&self) -> I {
        (self.relation)(self.current_number)
    }

    /// Calculates the next number in the sequence and updates the struct
    pub fn next(&mut self) {
        self.current_number = self.calculate_next();
    }

    /// Calls [`Relation::next`] `index` times, and then returns the result
    pub fn nth(&mut self, index: usize) -> I {
        for _ in 0..index {
            self.next();
        }

        self.current_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation() {
        let mut relation = Relation::new(1, |x| x + 1);

        assert_eq!(relation.current_number, 1);

        assert_eq!(relation.calculate_next(), 2);

        relation.next();

        assert_eq!(relation.current_number, 2);

        assert_eq!(relation.nth(100), 102)
    }

    #[test]
    fn test_float_relation() {
        let mut relation = Relation::new(1.0, |x| x + 1.);

        assert_eq!(relation.current_number, 1.);

        assert_eq!(relation.calculate_next(), 2.);

        relation.next();

        assert_eq!(relation.current_number, 2.);

        assert_eq!(relation.nth(100), 102.)
    }

    #[test]
    fn test_division() {
        let mut relation = Relation::new(1., |x| x / 2.);

        assert_eq!(relation.current_number, 1.);

        assert_eq!(relation.calculate_next(), 0.5);

        relation.next();

        assert_eq!(relation.current_number, 0.5);

        assert_eq!(relation.nth(100), 3.944304526105059_e-31);
    }

    #[test]
    fn test_multiplication() {
        let mut relation = Relation::new(1., |x| x * 2.);

        assert_eq!(relation.current_number, 1.);

        assert_eq!(relation.calculate_next(), 2.);

        relation.next();

        assert_eq!(relation.current_number, 2.);

        assert_eq!(relation.nth(100), 2.535301200456459_e30);
    }
}
