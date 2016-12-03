use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

use specimen::Specimen;

#[derive(PartialEq, PartialOrd, Debug)]
struct Selected {
    pub w: f64,
    pub i: usize,
}

impl Ord for Selected {
    fn cmp(&self, other: &Selected) -> Ordering {
        self.partial_cmp(other).unwrap().reverse()
    }
}

impl Eq for Selected { }

pub fn biggest(n: usize, specimina: &[Specimen]) -> Vec<usize> {
    if specimina.is_empty() || n == 0 {
        return Vec::new();
    }

    let mut biggest = BinaryHeap::with_capacity(n);
    
    biggest.extend(specimina.iter().enumerate().take(n).map(|(i, s)| Selected { w: s.fitness, i: i }));

    for s in specimina.iter().enumerate().skip(n).map(|(i, s)| Selected { w: s.fitness, i: i }) {
        if s > *biggest.peek().unwrap() {
            biggest.pop();
            biggest.push(s);
        }
    }

    return Vec::from_iter(biggest.into_iter().map(|s| s.i));
}

#[cfg(test)]
mod tests {
    use super::biggest;
    use specimen::Specimen;

    fn s(f: f64) -> Specimen {
        Specimen{ id: 1, fitness: 10.0, dna: Vec::new() }
    }

    #[test]
    fn biggest_of_none() {
        let b = biggest(1, &[]);

        assert!(b.is_empty());
    }
    
    #[test]
    fn biggest_none() {
        let b = biggest(0, &[s(1.0)]);

        println!("{:?}", b);
        assert!(b.is_empty());
    }

    #[test]
    fn biggest_one() {
        let b = biggest(1, &[s(1.0)]);

        assert_eq!(b.len(), 1);
    }
   
    #[test]
    fn biggest_three_of_size() {
        let mut b = biggest(3, &[s(5.0), s(1.0), s(4.0), s(2.0), s(0.0), s(3.0)]);
        b.sort();
        let expected = vec![0, 2, 5];
        
        assert_eq!(b, expected);
    }
}
