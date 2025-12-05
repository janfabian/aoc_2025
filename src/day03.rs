use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::read::Solution;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.chars().map(|n| n.to_digit(10).unwrap() as u8).collect())
        .collect()
}

pub struct PartA {
    matrix: Vec<Vec<u8>>,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        PartA {
            matrix: parse(input),
        }
    }

    fn execute(&self) {
        let mut result = BigInt::zero();

        for row in &self.matrix {
            let mut max_row = 0;
            let mut max_i: usize = 0;

            for (i, n) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    break;
                }

                if max_row < *n {
                    max_i = i;
                    max_row = *n;
                }
            }

            let mut next_max_row = 0;

            for i in (max_i + 1)..row.len() {
                if i == max_i {
                    continue;
                }
                if next_max_row < row[i] {
                    next_max_row = row[i];
                }
            }

            println!("Max: {} at {}, Next Max: {}", max_row, max_i, next_max_row);

            // sprintf

            let combined = BigInt::from_str(&format!("{}{}", max_row, next_max_row)).unwrap();
            result += combined;
        }

        println!("Result: {}", result);
    }
}
pub struct PartB {
    matrix: Vec<Vec<u8>>,
    length: usize,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {
            matrix: parse(input),
            length: 12,
        }
    }

    fn execute(&self) {
        let mut result = BigInt::zero();

        for row in &self.matrix {
            let mut i_g = 0;
            let mut row_n = String::new();
            for i in 0..self.length {
                let remaining = self.length - 1 - i;
                let mut max = row[i_g];
                let mut max_i = i_g;

                loop {
                    if i_g >= row.len() - remaining {
                        break;
                    }
                    if max < row[i_g] {
                        max = row[i_g];
                        max_i = i_g;
                    }

                    i_g += 1;
                }

                i_g = max_i + 1;
                row_n.push_str(&format!("{}", max));
            }

            println!("Row number: {}", row_n);
            let combined = BigInt::from_str(&row_n).unwrap();
            result += combined;
        }

        println!("Result: {}", result);
    }
}
