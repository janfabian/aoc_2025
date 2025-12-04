use crate::read::Solution;

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: i32,
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().chars();
            let direction = match parts.next() {
                Some('L') => Direction::L,
                Some('R') => Direction::R,
                _ => return None,
            };
            let distance = parts.as_str().parse().ok()?;
            Some(Move {
                direction,
                distance,
            })
        })
        .collect()
}

pub struct Part1A {
    moves: Vec<Move>,
}

impl Solution for Part1A {
    fn new(input: &str) -> Self {
        let moves = parse(input);
        Part1A { moves }
    }

    fn execute(&self) {
        let moves = &self.moves;
        let mut position = 50;
        let mut result = 0;
        for &Move {
            direction,
            distance,
        } in moves
        {
            position += match direction {
                Direction::L => -distance,
                Direction::R => distance,
            };

            position = (position + 100) % 100;

            if position == 0 {
                result += 1;
            }
        }

        println!("Parsed moves: {:?}", result);
    }
}

pub struct Part1B {
    moves: Vec<Move>,
}

impl Solution for Part1B {
    fn new(input: &str) -> Self {
        let moves = parse(input);
        Part1B { moves }
    }

    fn execute(&self) {
        let moves = &self.moves;
        let mut position = 50;
        let mut result = 0;
        for &Move {
            direction,
            distance,
        } in moves
        {
            let mut distance = distance;
            let mut move_result = 0;
            let begin_position = position;
            let rotations = distance / 100;

            distance %= 100;
            position += match direction {
                Direction::L => -distance,
                Direction::R => distance,
            };

            if begin_position > 0 && position <= 0 {
                // crossed zero going left
                move_result += 1;
            }

            if begin_position > 0 && position >= 100 {
                // crossed 100 going right
                move_result += 1;
            }

            move_result += rotations;
            position = (position + 100) % 100;

            result += move_result;
        }

        println!("Parsed moves: {:?}", result);
    }
}
