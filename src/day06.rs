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
pub struct PartA {
    matrix: Matrix,
    ops: Ops,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        let (mut matrix, ops) = parse(input);
        matrix.transpose();

        PartA {
            matrix: matrix,
            ops: ops,
        }
    }

    fn execute(&self) {
        let mut result = BigInt::zero();
        for (i, row) in self.matrix.data.iter().enumerate() {
            let op = &self.ops[i];

            let row_result = row
                .iter()
                .skip(1)
                .fold(row[0].clone(), |acc, x| op.apply(&acc, x));

            println!("Row {}: Result = {}", i, row_result);
            result += row_result;
        }

        println!("Final Result: {}", result);
    }
}
pub struct PartB {
    matrix: matrix::Matrix<char>,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {
            matrix: matrix::Matrix::from(input),
        }
    }

    fn execute(&self) {
        let ops = self.matrix.last_row().unwrap();
        let mut last_op: Option<Op> = None;
        let mut numbers: Vec<BigInt> = vec![];
        let mut result = BigInt::zero();
        for (y, c) in ops.iter().enumerate() {
            if *c != ' ' {
                if let Some(last_op) = &last_op {
                    let row_result = last_op.apply_fold(&numbers);

                    println!("Row {}: Result = {}", y, row_result);
                    result += row_result;
                }

                let op = op::Op::from_char(*c);
                last_op = Some(op);
                numbers.clear();
            }

            let mut n = vec![];
            for x in 0..self.matrix.num_rows() - 1 {
                let cell = self.matrix.get(x, y).unwrap();
                if *cell == ' ' {
                    continue;
                }
                n.push(cell.to_digit(10).unwrap());
            }
            if n.len() > 0 {
                numbers.push(BigInt::from_vec_of_digits(&n));
            }
        }

        let row_result = last_op.unwrap().apply_fold(&numbers);

        println!("Last row: Result = {}", row_result);
        result += row_result;

        println!("Final Result: {}", result);
    }
}
