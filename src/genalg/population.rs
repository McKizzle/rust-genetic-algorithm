extern crate rand;

use selected::biggest;
use specimen::Specimen;
use item::Item;

use rand::Rng;

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
    pub fn new(initial_pop_size: usize, 
               max_elites: usize, 
               max_pop_size: usize, 
               dna_length: usize) -> Population {
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
    pub fn cyccle(&mut self, 
                  items: &Vec<Item>, 
                  mutation_rate: f64, 
                  prioritize_elites: bool, 
                  protect_elites: bool) { 
        // 1: Calculate the fitnesses. 
        self.calculate_fitnesses(items);
        
        // 2: Create the offspring. 
        self.create_offspring(127, 256);
        self.calculate_fitnesses(items);
        
        // 3: Kill off population memebers based on their fitness. 
        if protect_elites {
            let max_elites = self.max_elites;
            self.run_natural_selection(max_elites);
        } else {
            self.run_natural_selection(0);
        }
    }
    
    /**
     * Updates the fitness values of a given vector of speciman.
     */
    fn calculate_fitnesses(&mut self, items: &Vec<Item>) {
        self.population = self.population.iter()
            .cloned()
            .map(|mut p| {
                p.fitness(items);
                p
            }).collect();
    }

    /**
     * Create the offspring of the items. 
     */
    fn create_offspring(&mut self, cand_pool_size: usize, max_offspring: usize) {
        let cps = if cand_pool_size >= self.population.len() {
            self.population.len()
        } else {
            cand_pool_size
        };

        let new_pop: Vec<Specimen> = if cps > 0 {
            let mating_candidates: &[Specimen] = &self.population[0..cps];
            mate_candidates(mating_candidates, max_offspring)
        } else {
            self.population.clone()
        };

        self.population = new_pop;
    }

    /**
     * Kill off unfit members of the population. 
     */
    fn run_natural_selection(&mut self, protect_top_n: usize) {
        let total_fitness: f64 = self.population.iter().map(|s| s.fitness).sum();
        let mut total_prob = self.population.iter().map(|s| s.fitness / total_fitness).sum();

        let mut survivors: Vec<Specimen> = self.population
            .iter()
            .filter(|s| {
                let prob = s.fitness / total_fitness;
                total_prob -= prob;
                return rand::thread_rng().next_f64() <= total_prob;
            })
            .cloned()
            .map(|mut s| {
                s.mutate(3f64/50f64);
                s
            })
            .collect();


        let best = biggest(protect_top_n, &self.population);
        let mut elites: Vec<Specimen> = Vec::new();
        for &i in best.iter() {
            elites.push(self.population[i].clone());
        }

        self.population = survivors.iter()
            .chain(elites.iter())
            .cloned()
            .collect();
    }

    pub fn get_most_fit(&self) -> Option<&Specimen> {
        let best = biggest(1, &self.population);
        return self.population.get(best[0]);
    }
}

/**
 * Mate all of the candidates and create new children. 
 */
fn mate_candidates(candidates: &[Specimen], max_offspring: usize) -> Vec<Specimen> {
    let mut children: Vec<Specimen> = Vec::new();

    while children.len() < max_offspring {
        let parent1 = match rand::thread_rng().choose(&candidates) {
            Some(x) => x,
            None => continue,
        };
        let parent2 = match rand::thread_rng().choose(&candidates) {
            Some(x) => x,
            None => continue,
        };

        children.push(Specimen::procreate(parent1, parent2).unwrap());
    }

    return children;
}
