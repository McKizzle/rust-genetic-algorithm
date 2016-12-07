use std::fmt;
use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use item::Item;
use std::cmp::Ordering;

#[derive(Debug)]
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

impl Ord for Specimen {
    fn cmp(&self, other: &Specimen) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Specimen {
    fn partial_cmp(&self, other: &Specimen) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl PartialEq for Specimen {
    fn eq(&self, other: &Specimen) -> bool {
        self.dna.eq(&other.dna)
    }
}

impl Eq for Specimen {}

impl fmt::Display for Specimen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Specimen {{\n");
        let _ = write!(f, " id: {}\n", self.id);
        let _ = write!(f, " fitness: {}\n", self.fitness);
        let _ = write!(f, " dna: [");
        for gene in &self.dna[0..&self.dna.len() - 1] {
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
    pub fn procreate<'a>(mate1: &'a Specimen, mate2: &'a Specimen) -> Result<Specimen, &'a str> {
        let take_amount = mate1.dna.len() - (mate1.dna.len() / 2);
        match mate1.dna.len() == mate2.dna.len() {
            true => Ok(Specimen { id: 1,
                                  dna: mate1.dna
                                            .iter()
                                            .take(take_amount)
                                            .chain(mate2.dna.iter()
                                                            .rev()
                                                            .take(mate2.dna.len() - take_amount))
                                            .cloned()
                                            .collect::<Vec<bool>>(),
                                  fitness: 0.0 }),
            false => Err("The dna lengths of the mates do not match"),
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
        let weight: f64 = self.dna
            .iter()
            .zip(items.iter())
            .filter(|t| *t.0)
            .map(|t| t.1)
            .map(|i| i.weight as f64)
            .sum();
        let value: f64 = self.dna
            .iter()
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

#[cfg(test)]
mod tests {
    use super::Specimen;

    #[test]
    fn id_correct_value() {
        let s = Specimen::new(1, 0);
        assert_eq!(s.id, 1);
    } 

    #[test]
    fn dna_correct_length_0() {
        let expected = 0;
        let s = Specimen::new(1, expected);
        assert_eq!(s.dna.len(), expected);
    }
    
    #[test]
    fn dna_correct_length_10() {
        let expected = 10;
        let s = Specimen::new(1, expected);
        assert_eq!(s.dna.len(), expected);
    }
    
    #[test]
    fn procreate_id() {
        let s1 = Specimen::new(1, 0);
        let s2 = Specimen::new(1, 0);
        
        let expected = Specimen{ id: 1, dna: Vec::new(), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.id, expected.id);
    }
    
    #[test]
    fn procreate_fitness() {
        let s1 = Specimen::new(1, 0);
        let s2 = Specimen::new(1, 0);
        
        let expected = Specimen{ id: 1, dna: Vec::new(), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.fitness, expected.fitness);
    }
    
    #[test]
    fn procreate_dna_len_0() {
        let s1 = Specimen::new(1, 0);
        let s2 = Specimen::new(1, 0);
        
        let expected = Specimen{ id: 1, dna: Vec::new(), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.dna, expected.dna);
    }
    
    #[test]
    fn procreate_dna_len_1() {
        let s1 = Specimen{ id: 1, dna: vec!(true), fitness: 0.0 };
        let s2 = Specimen{ id: 1, dna: vec!(false), fitness: 0.0 };
        
        let expected = Specimen{ id: 1, dna: vec!(true), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.dna, expected.dna);
    }

    #[test]
    fn procreate_dna_len_2() {
        let s1 = Specimen{ id: 1, dna: vec!(true, true), fitness: 0.0 };
        let s2 = Specimen{ id: 1, dna: vec!(false, false), fitness: 0.0 };
        
        let expected = Specimen{ id: 1, dna: vec!(true, false), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.dna, expected.dna);
    }

    #[test]
    fn procreate_dna_even_length() {
        let s1 = Specimen{ id: 1, dna: vec!(true, true, true, true), fitness: 0.0 };
        let s2 = Specimen{ id: 1, dna: vec!(false, false, false, false), fitness: 0.0 };

        let expected = Specimen{ id: 1, dna: vec!(true, true, false, false), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.dna, expected.dna);
    }
    
    #[test]
    fn procreate_dna_even_odd() {
        let s1 = Specimen{ id: 1, dna: vec!(true, true, true, true, true), fitness: 0.0 };
        let s2 = Specimen{ id: 1, dna: vec!(false, false, false, false, false), fitness: 0.0 };

        let expected = Specimen{ id: 1, dna: vec!(true, true, true, false, false), fitness: 0.0 };
        let actual = Specimen::procreate(&s1, &s2).unwrap();

        assert_eq!(actual.dna, expected.dna);
    }

    #[test]
    fn procreate_dna_len_mismatch() {
        let s1 = Specimen{ id: 1, dna: vec!(true, true, true, true), fitness: 0.0 };
        let s2 = Specimen{ id: 1, dna: vec!(false, false, false, false, false), fitness: 0.0 };

        assert_eq!(Specimen::procreate(&s1, &s2), Err("The dna lengths of the mates do not match"));
    }

    
}
