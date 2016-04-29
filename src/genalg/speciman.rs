pub use self::speciman::Speciman;

mod speciman {
    extern crate rand;
    
    use std::fmt;
    use self::rand::Rng;

    pub struct Speciman {
        pub id: i32,
        pub dna: Vec<bool>,
    }

    impl Clone for Speciman {
        fn clone(&self) -> Speciman {
            Speciman { 
                id: self.id, 
                dna: self.dna.iter().cloned().collect(),
            }
        }

        fn clone_from(&mut self, source: &Speciman) {
            unimplemented!(); 
        } 
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
         * Generates a new Speciman using two existing speciman. Single point crossover is used. 
         */
        pub fn procreate(mate1: Speciman, mate2: Speciman) -> Speciman {
            Speciman { 
                id: 1, 
                dna: mate1.dna.iter()
                              .take(mate1.dna.len() / 2)
                              .chain(mate2.dna.iter()
                                              .rev()
                                              .take(mate2.dna.len() / 2))
                              .cloned()
                              .collect::<Vec<bool>>(),
            }
        }

        /**
         * The fitness of the speciman.
         */
        pub fn fitness() -> i32 {
            0
        }
       
        /**
         * TODO: Mutation function for a speciman. 
         */
        pub fn mutate() {
            
        }
    }
}
