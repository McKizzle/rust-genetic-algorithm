pub use self::specimen::Specimen;

mod specimen {
    extern crate rand;
    
    use std::fmt;
    use self::rand::Rng;
    use item::Item;

    pub struct Specimen {
        pub id: i32,
        pub dna: Vec<bool>,
    }

    impl Clone for Specimen {
        fn clone(&self) -> Specimen {
            Specimen { 
                id: self.id, 
                dna: self.dna.iter().cloned().collect(),
            }
        }

        fn clone_from(&mut self, source: &Specimen) {
            unimplemented!(); 
        } 
    }

    impl fmt::Display for Specimen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let _ = write!(f, "Specimen {}: [", self.id);
            for gene in &self.dna[0 .. &self.dna.len() - 1] {
                let _ = write!(f, "{}, ", gene);
            }
            write!(f, "{}]", &self.dna[self.dna.len() - 1])
        }
    }

    impl Specimen {
        pub fn new(id: i32, dna_len: usize) -> Specimen { 
            Specimen { 
                id: id, 
                dna: rand::thread_rng().gen_iter::<bool>().take(dna_len).collect::<Vec<bool>>(),
            }
        }

        /**
         * Generates a new Specimen using two existing specimen. Single point crossover is used. 
         */
        pub fn procreate(mate1: Specimen, mate2: Specimen) -> Specimen {
            Specimen { 
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
         * The fitness of the specimen. The penalty is applied based on the 
         * square of the weight.
         *
         * TODO: It would be better if items was a vector of iterables. Then
         *   it would be more dynamic and not limited to an Item type. 
         */
        pub fn fitness(&self, items: &Vec<Item>) -> f64 {
            /*let weight: f64 =*/ 
            let weight: f64 = self.dna.iter()
                                       .zip(items.iter())
                                       .filter(|t| *t.0)
                                       .map(|t| t.1)
                                       .map(|i| i.weight as f64)
                                       .sum();
            let value: f64 = self.dna.iter()
                                     .zip(items.iter())
                                     .filter(|t| *t.0)
                                     .map(|t| t.1)
                                     .map(|i| i.value as f64)
                                     .sum();

            return value / weight.powf(2.0);
        }
       
        /**
         * TODO: Mutation function for a specimen. 
         */
        pub fn mutate() {
            
        }
    }
}
