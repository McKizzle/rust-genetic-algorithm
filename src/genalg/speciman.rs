pub use self::speciman::Speciman;

mod speciman {
    extern crate rand;
    
    use std::fmt;
    use self::rand::Rng;

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

    impl Speciman {
        pub fn new(id: i32, dna_len: usize) -> Speciman { 
            Speciman { 
                id: id, 
                dna: rand::thread_rng().gen_iter::<bool>().take(dna_len).collect::<Vec<bool>>(),
            }
        }

        /**
         * The fitness of the speciman.
         */
        pub fn fitness() -> i32 {
            0
        }
    }
}
