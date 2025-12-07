pub const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
    (-1, 0), // Up
];

pub const ADJACENT_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn transpose(&mut self) -> ()
    where
        T: Clone,
    {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        let mut transposed_data = vec![vec![]; cols];

        for r in 0..rows {
            for c in 0..cols {
                transposed_data[c].push(self.data[r][c].clone());
            }
        }

        self.data = transposed_data;
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn last_row(&self) -> Option<&Vec<T>> {
        self.data.last()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for item in row {
                write!(f, "{:?} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<Vec<char>>> for Matrix<char> {
    fn from(data: Vec<Vec<char>>) -> Self {
        Matrix { data }
    }
}

impl From<&str> for Matrix<char> {
    fn from(s: &str) -> Self {
        let data: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        Matrix { data }
    }
}

pub fn empty_matrix<T: Clone>(rows: usize, cols: usize, default: T) -> Matrix<T> {
    let data = vec![vec![default; cols]; rows];
    Matrix { data }
}
