pub use self::specimen::Specimen;

mod specimen {
    extern crate rand;
    
    use std::fmt;
    use self::rand::Rng; // why is 'self' needed before??
    use self::rand::distributions::{IndependentSample, Range};
    use item::Item;

    pub struct Specimen {
        pub id: i32,
        pub dna: Vec<bool>,
        pub fitness: f64,
    }

    impl Clone for Specimen {
        fn clone(&self) -> Specimen {
            Specimen { 
                id: self.id, 
                dna: self.dna.iter().cloned().collect(),
                fitness: 0.0,
            }
        }

        fn clone_from(&mut self, source: &Specimen) {
            unimplemented!(); 
        } 
    }

    impl fmt::Display for Specimen {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let _ = write!(f, "Specimen {{\n");
            let _ = write!(f, " id: {}\n", self.id);
            let _ = write!(f, " fitness: {}\n", self.fitness);
            let _ = write!(f, " dna: [");
            for gene in &self.dna[0 .. &self.dna.len() - 1] {
                let _ = write!(f, "{}, ", *gene as i32);
            }
            let _ = write!(f, "{}]\n", *&self.dna[self.dna.len() - 1] as i32);
            write!(f, "}}")
        }
    }

    impl Specimen {
        pub fn new(id: i32, dna_len: usize) -> Specimen { 
            Specimen { 
                id: id, 
                dna: rand::thread_rng().gen_iter::<bool>().take(dna_len).collect::<Vec<bool>>(),
                fitness: 0.0,
            }
        }

        /**
         * Generates a new Specimen using two existing specimen. Single point crossover is used. 
         */
        pub fn procreate(mate1: &Specimen, mate2: &Specimen) -> Specimen {
            Specimen { 
                id: 1, 
                dna: mate1.dna.iter()
                              .take(mate1.dna.len() / 2)
                              .chain(mate2.dna.iter()
                                              .rev()
                                              .take(mate2.dna.len() / 2))
                              .cloned()
                              .collect::<Vec<bool>>(),
                fitness: 0.0,
            }
        }

        /**
         * The fitness of the specimen. The penalty is applied based on the 
         * square of the weight.
         *
         * TODO: It would be better if items was a vector of iterables. Then
         *   it would be more dynamic and not limited to an Item type. 
         */
        pub fn fitness(&mut self, items: &Vec<Item>) {
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

            self.fitness = value / weight.powf(2.0);
        }
       
        /**
         * TODO: Mutation function for a specimen. 
         */
        pub fn mutate(&mut self, mutation_rate: f64) {
            let mut i = self.geometric_distribution(mutation_rate) as usize;

            while i < self.dna.len() {
                i += self.geometric_distribution(mutation_rate) as usize;

                println!("i = {}", i);
            }
        }

        fn geometric_distribution(&self, prob: f64) -> i32 {
            let min: f64 = 0.0;
            let max: f64 = 2.0f64.powf(32f64);

            let between = Range::new(min, max);
            let mut rng = rand::thread_rng();

            let uniform: f64 = between.ind_sample(&mut rng) / max;

            let b10: f64 = 10f64;
            let gd: f64 = (uniform.log(b10) / (1.0 - prob).log(b10)).ceil();

            return gd as i32;
        }
    }
}
