pub use self::speciman::Speciman;

mod speciman {
    use std::fmt;

    pub struct Speciman {
        pub id: i32,
        pub dna: Vec<bool>,
    }

    impl fmt::Display for Speciman {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let _ = write!(f, "Speciman {}: [", self.id);
            for gene in &self.dna[0 .. &self.dna.len() - 1] {
                let _ = write!(f, "{}, ", gene);
            }
            write!(f, "{}]", &self.dna[self.dna.len() - 1])
        }
    }
}
