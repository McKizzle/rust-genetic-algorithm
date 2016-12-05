extern crate genalg;
extern crate time;
extern crate ordered_float;
extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use genalg::specimen::Specimen;
use genalg::item::Item;
use genalg::selected::biggest;

use ordered_float::OrderedFloat;

use rand::Rng;

pub fn main() {
    let items_file: File = match File::open("data/items.txt") {
        Err(why) => panic!("Failed to open the file: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut items: Vec<Item> = Vec::new();
    let reader: BufReader<File> = BufReader::new(items_file);
    for line in reader.lines() {
        let l = line.unwrap();
        let itm: Vec<i32> = l.split(',')
            .map(|s| match s.parse::<i32>() {
                Ok(n) => n,
                Err(_) => -1,
            })
            .filter(|&x| x >= 0)
            .collect();

        match itm.len() {
            2 => {
                items.push(Item {
                    weight: itm[0],
                    value: itm[1],
                })
            }
            _ => {}
        };
    }

    println!("Items available for the taking");
    for i in &items {
        println!("{}", i);
    }
    println!("------------------------------");

    println!("Calculating most efficient theft...");
    let mut result: Specimen = simulate(&items, 2048);
    result.fitness(&items);
    println!("...Done");
    println!("A winner was found! {}.", result);
}

fn simulate(items: &Vec<Item>, pop_size: i32) -> Specimen {
    let mut_rate: f64 = 3.0 / 100.0 * items.len() as f64;

    // Create the inital population.
    let mut specimina: Vec<Specimen> =
        (0..pop_size).map(|i| Specimen::new(i, items.len())).collect();


    let t0_s: f64 = time::precise_time_s();
    for i in 0..1000 {
        println!("Iteration {}", i);
        for s in &mut specimina {
            s.fitness(items);
        }
        specimina.sort();
        specimina.reverse();

        //        let tmp =

        let offspring = create_offspring(items, &specimina, 127, 256);
        specimina.extend(offspring.iter().cloned());

        run_natural_selection(&mut specimina, 10, 1024);
    }
    let dt_s: f64 = time::precise_time_s() - t0_s;
    println!("Total time: {}", dt_s);

    return match specimina.get(0) {
        Some(s) => s.clone(),
        None => Specimen::new(0, 0),
    };
}

fn run_natural_selection(specimina: &mut Vec<Specimen>, n: usize, max_allowed_size: usize) {
    specimina.sort();
    specimina.reverse();

    let total_fitness: f64 = specimina.iter().map(|s| s.fitness).sum();
    let mut total_prob = specimina.iter().map(|s| s.fitness / total_fitness).sum();

    let mut survivors: Vec<Specimen> = Vec::new();
    survivors.extend(specimina.iter()
        .filter(|s| {
            let prob = s.fitness / total_fitness;
            total_prob -= prob;
            return rand::thread_rng().next_f64() <= total_prob;
        })
        .cloned());

    let mut elites: Vec<Specimen> = specimina.drain(0..n).collect();
    survivors.extend(elites.iter().cloned());

    specimina.clear();
    specimina.extend(survivors.iter().cloned());
}

fn create_offspring(items: &Vec<Item>,
                    specimina: &Vec<Specimen>,
                    cand_pool_size: usize,
                    max_offspring: usize)
                    -> Vec<Specimen> {
    let cps = if cand_pool_size >= specimina.len() {
        specimina.len()
    } else {
        cand_pool_size
    };

    let mating_candidates: &[Specimen] = &specimina[0..cps];
    let mut new_pop: Vec<Specimen> = mate_population(mating_candidates, max_offspring);

    for s in &mut new_pop {
        s.fitness(items);
    }

    return new_pop;
}

fn mate_population(population: &[Specimen], max_offspring: usize) -> Vec<Specimen> {
    let mut children: Vec<Specimen> = Vec::new();

    while children.len() < max_offspring {
        let parent1 = match rand::thread_rng().choose(&population) {
            Some(x) => x,
            None => continue,
        };
        let parent2 = match rand::thread_rng().choose(&population) {
            Some(x) => x,
            None => continue,
        };

        children.push(Specimen::procreate(parent1, parent2));
    }

    return children;
}
