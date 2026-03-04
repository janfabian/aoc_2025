use std::vec;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::{
    bigint::BigIntExt,
    matrix::{self},
    op::{self, Op},
    read::Solution,
};

fn parse(input: &str) -> (Matrix, Ops) {
    let lines_n = input.lines().count();
    let data = input
        .lines()
        .take(lines_n - 1)
        .map(|s| {
            s.split_whitespace()
                .flat_map(|n| BigInt::parse_bytes(n.as_bytes(), 10))
                .collect()
        })
        .collect();

    let ops = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .map(Op::from_char)
        .collect();

    (Matrix { data: data }, ops)
}

type Matrix = matrix::Matrix<BigInt>;
type Ops = Vec<Op>;

//enum
pub struct PartA {}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        PartA {}
    }

    fn execute(&self) {}
}
pub struct PartB {}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {}
    }

    fn execute(&self) {}
}
