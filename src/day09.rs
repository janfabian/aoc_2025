use std::fs::File;
use std::io::Write;

use crate::{
    coords::{Coords, Grid},
    read::Solution,
};

fn parse(input: &str) -> Vec<Coords<u64>> {
    return input
        .lines()
        .filter_map(|s| {
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
                //     self.grid.coords[i].x,
                //     self.grid.coords[i].y,
                //     self.grid.coords[j].x,
                //     self.grid.coords[j].y,
                //     area
                // );

                if area > result {
                    result = area;
                }
            }
        }

        println!("Max area = {}", result);

        // println!("{}", self.grid);
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
                // println!("============");

                let area = self.grid.coords[i].area(&self.grid.coords[j], true);
                if area > result {
                    let (p1, p2) = (&self.grid.coords[i], &self.grid.coords[j]);
                    let rect = Coords::rectangle(&self.grid.coords[i], &self.grid.coords[j]);

                    for i in 0..self.grid.coords.len() {
                        let (c1, c2) = (
                            &self.grid.coords[i],
                            &self.grid.coords[(i + 1) % self.grid.coords.len()],
                        );

                        let right = p1.x.max(p2.x) <= c1.x.min(c2.x);
                        let left = p1.x.min(p2.x) >= c1.x.max(c2.x);
                        let up = p1.y.max(p2.y) <= c1.y.min(c2.y);
                        let down = p1.y.min(p2.y) >= c1.y.max(c2.y);

                        if !(right || left || up || down) {
                            continue 'next;
                        }
                    }

                    // for coord in &rect {
                    //     let is_in =
                    //         self.grid.is_on_boundary(&coord) || self.grid.is_in_shape(&coord);

                    //     if !is_in {
                    //         continue 'next;
                    //     }
                    // }

                    println!("{}", area);

                    result = area;
                }
            }
        }

        println!("Max enclosed area = {}", result);
        // println!("{}", self.grid);
    }
}
