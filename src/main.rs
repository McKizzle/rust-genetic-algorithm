extern crate genalg;

use genalg::speciman::Speciman;
use genalg::item::Item;

pub fn main() {
    let dna: Vec<bool> = vec![false, true, true, false];
    let a_speciman: Speciman = Speciman { id: 1, dna: dna };
    println!("A Fine Speciman: {}", a_speciman);

    let itm: Item = Item { weight: 1, value: 2 };
    println!("A Fine Item: {}", itm);
}
