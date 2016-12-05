use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

use specimen::Specimen;

#[derive(Debug)]
struct Selected {
    pub w: f64,
    pub i: usize,
}

impl Ord for Selected {
    fn cmp(&self, other: &Selected) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}

impl PartialOrd for Selected {
    fn partial_cmp(&self, other: &Selected) -> Option<Ordering> {
        other.w.partial_cmp(&self.w)
    }
}

impl Eq for Selected {}

impl PartialEq for Selected {
    fn eq(&self, other: &Selected) -> bool {
        self.w == other.w
    }
}

pub fn biggest(n: usize, specimina: &[Specimen]) -> Vec<usize> {
    if specimina.is_empty() || n == 0 {
        return Vec::new();
    }

    let mut biggest = BinaryHeap::with_capacity(n);

    biggest.extend(specimina.iter().enumerate().take(n).map(|(i, s)| {
        Selected {
            w: s.fitness,
            i: i,
        }
    }));

    for s in specimina.iter().enumerate().skip(n).map(|(i, s)| {
        Selected {
            w: s.fitness,
            i: i,
        }
    }) {
        if s < *biggest.peek().unwrap() {
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
        Specimen {
            id: 1,
            fitness: f,
            dna: Vec::new(),
        }
    }

    #[test]
    fn biggest_of_none() {
        let b = biggest(1, &[]);

        assert!(b.is_empty());
    }

    #[test]
    fn biggest_none() {
        let b = biggest(0, &[s(1.0)]);

        assert!(b.is_empty());
    }

    #[test]
    fn biggest_one() {
        let b = biggest(1, &[s(1.0)]);

        assert_eq!(b.len(), 1);
    }

    #[test]
    fn biggest_three_of_size() {
        let mut splc: &[Specimen] = &[s(5.0), s(1.0), s(4.0), s(2.0), s(0.0), s(3.0)];
        let mut b = biggest(3, splc);
        b.sort();
        let expected = vec![0, 2, 5];

        assert_eq!(b, expected);
    }

    quickcheck! {
        fn same_as_sorting(n: usize, weights: Vec<f64>) -> bool {
            let mut spec: Vec<Specimen> = weights.into_iter().map(s).collect();
            // Fancy new method
            let mut fast = biggest(n, &spec);
            fast.sort();
            fast.reverse();
            // Original method
            let mut enumerated: Vec<_> = spec
                .iter()
                .enumerate()
                .collect();
            enumerated.sort_by_key(|e| e.1);
            let slow: Vec<_> = enumerated
                .into_iter()
                .rev()
                .map(|e| e.0)
                .take(n)
                .collect();
            // Should be the same
            fast == slow
        }
    }
}
