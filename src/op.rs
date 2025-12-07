use std::ops::{Add, Mul};

pub enum Op {
    Plus,
    Mul,
}

impl Op {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Op::Plus,
            "*" => Op::Mul,
            _ => panic!("Invalid operator string"),
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '+' => Op::Plus,
            '*' => Op::Mul,
            _ => panic!("Invalid operator character"),
        }
    }

    pub fn apply<'a, T>(&self, a: &'a T, b: &'a T) -> T
    where
        &'a T: Add<Output = T> + Mul<Output = T>,
    {
        match self {
            Op::Plus => a + b,
            Op::Mul => a * b,
        }
    }

    pub fn apply_fold<T>(&self, numbers: &[T]) -> T
    where
        for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
        T: Clone,
    {
        numbers
            .iter()
            .skip(1)
            .fold(numbers[0].clone(), |acc, x| self.apply(&acc, x))
    }
}
