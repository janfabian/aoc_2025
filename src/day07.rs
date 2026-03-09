use std::collections::{HashMap, HashSet};

use crate::read::Solution;

fn parse(input: &str) -> (usize, Vec<HashSet<usize>>) {
    let start = input
        .lines()
        .take(1)
        .flat_map(|s| s.find("S"))
        .collect::<Vec<usize>>()[0];
    let splitters: Vec<HashSet<usize>> = input
        .lines()
        .skip(1)
        .map(|s| {
            return s.match_indices("^").map(|(i, _)| i).collect();
        })
        .collect();

    (start, splitters)
}

//enum
pub struct PartA {
    start: usize,
    splitters: Vec<HashSet<usize>>,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        let (start, splitters) = parse(input);
        PartA { start, splitters }
    }

    fn execute(&self) {
        let mut beams = HashSet::from([self.start]);
        let mut splits: usize = 0;
        for line_splitters in &self.splitters {
            for splitter in line_splitters {
                if beams.contains(splitter) {
                    beams.remove(splitter);
                    beams.insert(*splitter + 1);
                    beams.insert(*splitter - 1);
                    splits += 1;
                }
            }
        }

        println!("splits {} ", splits);
    }
}
pub struct PartB {
    start: usize,
    splitters: Vec<HashSet<usize>>,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        let (start, splitters) = parse(input);
        PartB { start, splitters }
    }

    fn execute(&self) {
        let mut beams: Vec<HashSet<usize>> = vec![HashSet::from([self.start])];
        for line_splitters in &self.splitters {
            let prev_beams = beams.last().expect("beams cant be empty");
            let mut next_beams = HashSet::new();
            for splitter in line_splitters {
                if prev_beams.contains(splitter) {
                    next_beams.insert(*splitter + 1);
                    next_beams.insert(*splitter - 1);
                }
            }

            for beam in prev_beams {
                if !line_splitters.contains(beam) {
                    next_beams.insert(*beam);
                }
            }

            beams.push(next_beams);
        }

        let mut acc = HashMap::<usize, usize>::new();

        for i in (0..beams.len() - 1).rev() {
            let (current, next) = (beams.get(i).unwrap(), beams.get(i + 1).unwrap());

            for b in current {
                if next.contains(b) {
                    acc.entry(*b).or_insert(1);
                } else {
                    let (left_key, right_key) = ((*b - 1), (*b + 1));
                    if next.contains(&left_key) && next.contains(&right_key) {
                        acc.insert(
                            *b,
                            acc.get(&left_key).unwrap() + acc.get(&right_key).unwrap(),
                        );
                    }
                }
            }
        }

        println!("paths {:?} ", acc.get(&self.start));
    }
}
