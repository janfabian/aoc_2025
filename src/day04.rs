use crate::{
    matrix::{self, empty_matrix},
    read::Solution,
};

fn parse(input: &str) -> Matrix {
    let data = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|n| match n {
                    '@' => Coord::Paper,
                    '.' => Coord::Empty,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect();

    Matrix { data: data }
}

type Matrix = matrix::Matrix<Coord>;

//enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Coord {
    Paper,
    Empty,
}

pub struct PartA {
    matrix: Matrix,
    min_adjacent: usize,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        PartA {
            matrix: parse(input),
            min_adjacent: 4,
        }
    }

    fn execute(&self) {
        let mut adjacent_paper = empty_matrix(self.matrix.data.len(), self.matrix.data[0].len(), 0);
        for (row_idx, row) in self.matrix.data.iter().enumerate() {
            for (col_idx, coord) in row.iter().enumerate() {
                if coord == &Coord::Paper {
                    for dir in matrix::ADJACENT_DIRECTIONS.iter() {
                        let new_row = row_idx as isize + dir.0;
                        let new_col = col_idx as isize + dir.1;
                        if new_row >= 0
                            && new_row < self.matrix.data.len() as isize
                            && new_col >= 0
                            && new_col < self.matrix.data[0].len() as isize
                        {
                            adjacent_paper.data[new_row as usize][new_col as usize] += 1;
                        }
                    }
                }
            }
        }
        let mut result = 0;
        for (row_idx, row) in self.matrix.data.iter().enumerate() {
            for (col_idx, coord) in row.iter().enumerate() {
                if coord == &Coord::Paper
                    && adjacent_paper.data[row_idx][col_idx] < self.min_adjacent
                {
                    result += 1;
                }
            }
        }

        println!("Result: {result}");
    }
}
pub struct PartB {
    matrix: Matrix,
    min_adjacent: usize,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {
            matrix: parse(input),
            min_adjacent: 4,
        }
    }

    fn execute(&self) {
        let mut matrix_it = self.matrix.clone();
        let mut result = 0;
        loop {
            let mut adjacent_paper =
                empty_matrix(self.matrix.data.len(), matrix_it.data[0].len(), 0);
            for (row_idx, row) in matrix_it.data.iter().enumerate() {
                for (col_idx, coord) in row.iter().enumerate() {
                    if coord == &Coord::Paper {
                        for dir in matrix::ADJACENT_DIRECTIONS.iter() {
                            let new_row = row_idx as isize + dir.0;
                            let new_col = col_idx as isize + dir.1;
                            if new_row >= 0
                                && new_row < matrix_it.data.len() as isize
                                && new_col >= 0
                                && new_col < matrix_it.data[0].len() as isize
                            {
                                adjacent_paper.data[new_row as usize][new_col as usize] += 1;
                            }
                        }
                    }
                }
            }
            let mut has_changes = false;
            for row_idx in 0..matrix_it.data.len() {
                for col_idx in 0..matrix_it.data[row_idx].len() {
                    let coord = &matrix_it.data[row_idx][col_idx];
                    if coord == &Coord::Paper
                        && adjacent_paper.data[row_idx][col_idx] < self.min_adjacent
                    {
                        matrix_it.data[row_idx][col_idx] = Coord::Empty;
                        has_changes = true;
                        result += 1;
                    }
                }
            }

            if !has_changes {
                break;
            }
        }

        println!("Result: {result}");
    }
}
