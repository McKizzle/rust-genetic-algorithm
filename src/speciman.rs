use std::fmt;
use rand::Rng;

pub struct Speciman {
    pub id: i32,
    pub dna: Vec<bool>,
}

pub impl Speciman {
    pub fn new() -> Speciman {
        Speciman { 
            id: 1, 
            dna: Vec::new(),
        }
    }

    //fn new(id: i32, dna_len: u32) -> Speciman { 
    //    let mut rng = rand::thread_rng();
    //    Speciman {
    //        id: -1,
    //        dna: Vec::new(),
    //    }
    //}
    
    pub fn yelp() {
        println!("YELP");
    }
}

impl fmt::Display for Speciman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Speciman {}: [", self.id);
        for gene in &self.dna[0 .. &self.dna.len() - 1] {
            let _ = write!(f, "{}, ", gene);
        }
        write!(f, "{}]", &self.dna[self.dna.len() - 1])
    }
}

