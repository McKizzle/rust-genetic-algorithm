extern crate genalg;
extern crate time;
extern crate rand;

#[link(name = "-s EXPORTED_FUNCTIONS=['_external_main', '_hello_test']")]
#[link(name = "-s DEMANGLE_SUPPORT=1")]
extern {}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use genalg::population::Population;
use genalg::specimen::Specimen;
use genalg::item::Item;

pub fn main() {
    external_main("data/items.txt");
}

#[no_mangle]
pub fn hello_test() {
    println!("Testing Rust!");
}

#[no_mangle]
pub fn external_main(itemsPath: &str) {
    let items_file: File = match File::open(Path::new(itemsPath)) {
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
    println!("The total value of stolen goods was: {}", result.total_value(&items));
}

fn simulate(items: &Vec<Item>, pop_size: i32) -> Specimen {
    let mut_rate: f64 = 3.0 / 100.0 * items.len() as f64;

    let mut population = Population::new(pop_size as usize, 10, (pop_size * 2) as usize, items.len());

    let t0_s: f64 = time::precise_time_s();
    for _ in 0..1000 {
        population.cycle(items, mut_rate, true);
    }
    let dt_s: f64 = time::precise_time_s() - t0_s;
    println!("Total time: {}", dt_s);


    return match population.get_most_fit() {
        Some(s) => s.clone(),
        None => Specimen::new(0, 0),
    };
}

