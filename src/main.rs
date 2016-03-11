extern crate genalg;

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
    let result: Speciman = simulate(&items);
    println!("...Done");
    println!("A winner was found! {}.", result);
}

fn simulate(items: &Vec<Item>) -> Speciman {
    let mut testman: Speciman = Speciman::new(1, items.len());

    return testman;
}

