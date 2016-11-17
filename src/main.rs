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
        let itm: Vec<i32> = l.split(',').map(|s| match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => -1,
        }).filter(|&x| x >= 0).collect();

        match itm.len() {
            2 => items.push(Item {weight: itm[0], value: itm[1]}),
            _ => {},
        };
    }

    println!("Items available for the taking");
    for i in &items {
        println!("{}", i);
    }
    println!("------------------------------");
    
    println!("Calculating most efficient theft...");
    let mut result: Specimen = simulate(&items, 2048);
    println!("...Done");
    println!("A winner was found! {}.", result);
}

fn simulate(items: &Vec<Item>, pop_size: i32) -> Specimen {
    let mut_rate: f64 = 3.0 / 100.0 * items.len() as f64; 

    // Create the inital population. 
    let mut specimina: Vec<Specimen> = (0..pop_size).map(|i| { Specimen::new(i, items.len()) }).collect();


    let t0_s: f64 = time::precise_time_s();        
    for _ in 0..100 {
        for s in &mut specimina {
            s.fitness(items);
        }
        specimina.sort_by(|s1, s2| OrderedFloat(s1.fitness).cmp(&OrderedFloat(s2.fitness)).reverse());

        let mut new_pop: Vec<Specimen>; 
        {
            let mating_candidates: &[Specimen] = &specimina[0 .. 127];
            new_pop = mate_population(mating_candidates, 256);
            
            for s in &mut new_pop {
                s.fitness(items);
            }
        }
        //specimina.extend(new_pop.iter().map(|&s| s));
        
        break;
    }
    let dt_s: f64 = time::precise_time_s() - t0_s;        
    println!("Total time: {}", dt_s);

    return match specimina.get(0) {
        Some(s) => s.clone(),
        None => Specimen::new(0, 0),
    }
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
