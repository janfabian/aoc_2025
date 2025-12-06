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

pub fn empty_matrix<T: Clone>(rows: usize, cols: usize, default: T) -> Matrix<T> {
    let data = vec![vec![default; cols]; rows];
    Matrix { data }
}
