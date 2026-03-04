use crate::{
    coords::{Coords, Grid},
    read::Solution,
};

fn parse(input: &str) -> Vec<Coords<u64>> {
    return input
        .lines()
        .map(|s| {
            let coords = s
                .split(',')
                .flat_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<_>>();
            if coords.len() == 2 {
                Some([coords[0], coords[1]])
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|arr| Coords::from(arr))
        .collect();
}

//enum
pub struct PartA {
    grid: Grid<u64>,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        PartA {
            grid: Grid::from_coords(parse(input)),
        }
    }

    fn execute(&self) {
        let mut result = 0;
        for i in 0..self.grid.coords.len() - 1 {
            for j in i + 1..self.grid.coords.len() {
                let area = self.grid.coords[i].area(&self.grid.coords[j], true);
                // println!(
                //     "Area between ({}, {}) and ({}, {}) = {}",
                //     self.grid.coords[i].x, self.grid.coords[i].y, self.grid.coords[j].x, self.grid.coords[j].y, area
                // );

                if area > result {
                    result = area;
                }
            }
        }

        println!("Max area = {}", result);

        println!("{}", self.grid);
    }
}
pub struct PartB {
    grid: Grid<u64>,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        PartB {
            grid: Grid::from_coords(parse(input)),
        }
    }

    fn execute(&self) {
        let mut result = 0;
        for i in 0..self.grid.coords.len() - 1 {
            'next: for j in i + 1..self.grid.coords.len() {
                // self.grid
                //     .display_rectangle(&self.grid.coords[i], &self.grid.coords[j]);

                let rect = Coords::rectangle(&self.grid.coords[i], &self.grid.coords[j]);
                for coord in rect {
                    let is_in = self.grid.is_in_shape(coord.y);

                    if !is_in {
                        continue 'next;
                    }
                }

                let area = self.grid.coords[i].area(&self.grid.coords[j], true);
                if area > result {
                    result = area;
                }
            }
        }

        println!("Max enclosed area = {}", result);
    }
}
