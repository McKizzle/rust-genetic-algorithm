use specimen::Specimen;
use item::Item;

#[derive(Debug)]
pub struct Population {
    population: Vec<Specimen>,
    elites: Vec<Specimen>,
    max_elites: usize,
    max_pop_size: usize,
}

/**
 * Implementation for the population. 
 */
impl Population {
    /**
     * Creates a new population with the desired constraints. 
     */
    pub fn new(initial_pop_size: usize, max_elites: usize, max_pop_size: usize, dna_length: usize) -> Population {
        Population {
            population: (0..initial_pop_size as i32).map(|i| Specimen::new(i, dna_length)).collect(),
            elites: Vec::new(),
            max_elites: max_elites,
            max_pop_size: max_pop_size,
        }
    }

    /**
     * Runs a cycle on the population. 
     *
     * @param: the mutation rate for each nucliotide in each speciman's dna in the population. 
     * @param: prioritize elites in mating and survival.
     * @param: protect the elites from alterations or deletions. 
     */
    pub fn cyccle(&mut self, items: &Vec<Item>, mutation_rate: f64, prioritize_elites: bool, protect_elites: bool) { 
        // 1: Calculate the fitnesses. 
        self.calculate_fitnesses(items);
        
        // 2: Create the offspring. 
        
        // 3: Kill off population memebers based on their fitness. 
    }
    
    /**
     * Updates the fitness values of a given vector of speciman.
     */
    fn calculate_fitnesses(&mut self, items: &Vec<Item>) {
        for mut p in self.population.into_iter() {
            p.fitness(items);
        }
    }
}
