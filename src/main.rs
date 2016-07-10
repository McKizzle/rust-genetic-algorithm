extern crate genalg;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use genalg::speciman::Speciman;
use genalg::item::Item;

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
    let result: Speciman = simulate(&items, 2048);
    println!("...Done");
    println!("A winner was found! {}.", result);
}

fn simulate(items: &Vec<Item>, pop_size: i32) -> Speciman {
    let mut_rate: f64 = 3.0 / 100.0 * items.len() as f64; 

    // Create the inital population. 
    let mut specimina: Vec<Speciman> = (0..pop_size).map(|i| { Speciman::new(i, items.len()) }).collect();

    let t0_s: f64 = time::precise_time_s();        
    for j in 0..2500 {
        
    }
    let dt_s: f64 = time::precise_time_s() - t0_s;        

    return match specimina.get(0) {
        Some(s) => s.clone(),
        None => Speciman::new(0, 0),
    }
}

fn mutate_population() {}
