use std::ops::Div;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::read::Solution;

#[derive(Debug)]
struct Interval {
    start: BigInt,
    end: BigInt,
}

impl Interval {
    fn num_of_digits(&self) -> [usize; 2] {
        let start_digits = self.start.num_of_digits();
        let end_digits = self.end.num_of_digits();
        [start_digits, end_digits]
    }

    fn minvalue_of_digits(&self, digits: usize) -> BigInt {
        BigInt::from(10).pow(digits as u32 - 1)
    }

    fn maxvalue_of_digits(&self, digits: usize) -> BigInt {
        BigInt::from(10).pow(digits as u32) - 1u32
    }
}

trait BigIntExt {
    fn num_of_digits(&self) -> usize;
    fn vec_of_digits(&self) -> Vec<u8>;
    fn first_half(&self) -> Self;
    fn second_half(&self) -> Self;
    fn clone_double(&self) -> Self;
    fn check_for_pattern(&self) -> bool;
}

impl BigIntExt for BigInt {
    fn num_of_digits(&self) -> usize {
        self.to_string().len()
    }

    fn vec_of_digits(&self) -> Vec<u8> {
        self.to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect()
    }

    fn first_half(&self) -> Self {
        let digits = self.num_of_digits() / 2;

        self.div(BigInt::from(10).pow(digits as u32))
    }

    fn second_half(&self) -> Self {
        let digits = self.num_of_digits() / 2;

        let divisor = BigInt::from(10).pow(digits as u32);
        self % divisor
    }

    fn clone_double(&self) -> Self {
        let s = self.to_string();
        let doubled = format!("{}{}", s, s);
        BigInt::parse_bytes(doubled.as_bytes(), 10).unwrap()
    }

    fn check_for_pattern(&self) -> bool {
        let n = self.num_of_digits();
        let self_str = self.to_string();

        if n < 2 {
            return false;
        }

        for i in (1..=(n / 2)).rev() {
            if n % i != 0 {
                continue;
            }

            let pattern = &self_str.as_bytes()[0..i];

            let contains_only_pattern = self_str.as_bytes().chunks(i).all(|chunk| chunk == pattern);
            if contains_only_pattern {
                return true;
            }
        }

        return false;
    }
}

fn parse(input: &str) -> Vec<Interval> {
    input
        .split(",")
        .map(|s| {
            let mut parts = s.trim().split("-");
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Interval { start, end }
        })
        .collect()
}

pub struct PartA {
    intervals: Vec<Interval>,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        PartA {
            intervals: parse(input),
        }
    }

    fn execute(&self) {
        let mut result = BigInt::zero();
        for interval in &self.intervals {
            let digits = interval.num_of_digits();

            let min = if digits[0] % 2 == 0 {
                &interval.start
            } else {
                &interval.minvalue_of_digits(digits[0] + 1)
            };

            let max = if digits[1] % 2 == 0 {
                &interval.end
            } else {
                &interval.maxvalue_of_digits(digits[1] - 1)
            };

            println!(
                "Processing interval from {} to {} with adjusted bounds {} to {}",
                interval.start, interval.end, min, max
            );

            if min > max {
                println!("No valid numbers in this interval.");
                continue;
            }

            let mut first_half_it = min.first_half();
            let first_half_max = max.first_half();

            while first_half_it <= first_half_max {
                let add = &first_half_it.clone_double();

                first_half_it += 1;

                if add < min || add > max {
                    continue;
                }
                println!("adding {}", add);
                result += add;
            }
        }

        println!("Result: {}", result);
    }
}

pub struct PartB {
    intervals: Vec<Interval>,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {
            intervals: parse(input),
        }
    }

    fn execute(&self) {
        let mut result = BigInt::zero();
        for interval in &self.intervals {
            let digits = interval.num_of_digits();
            println!(
                "Interval from {} to {} has {} and {} digits respectively.",
                interval.start, interval.end, digits[0], digits[1]
            );

            let mut i = interval.start.clone();

            while i <= interval.end {
                if i.check_for_pattern() {
                    println!("Found pattern: {}", i);
                    result += &i;
                }
                i += 1;
            }
        }
        println!("Result: {}", result);
    }
}
