use std::{
    fmt::Display,
    ops::{Add, Mul, RangeBounds, Sub},
};

pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl From<[u64; 2]> for Coords<u64> {
    fn from(arr: [u64; 2]) -> Self {
        Coords {
            x: arr[0],
            y: arr[1],
        }
    }
}

impl<T> Coords<T> {
    pub fn area(&self, other: &Coords<T>, inclusive: bool) -> T
    where
        T: From<u32> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy,
    {
        let width = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let height = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        if inclusive {
            (width + T::from(1u32)) * (height + T::from(1u32))
        } else {
            width * height
        }
    }
}

pub struct Grid<T> {
    pub coords: Vec<Coords<T>>,
}

impl<T> Grid<T> {
    pub fn from_coords(coords: Vec<Coords<T>>) -> Self {
        Grid { coords }
    }
}

impl Grid<u64> {
    pub fn display_rectangle(&self, p1: &Coords<u64>, p2: &Coords<u64>) {
        let rectangle = Coords::rectangle(p1, p2);
        let bbox = self.bounding_box().unwrap();
        for y in bbox.0.y - 1..=bbox.1.y + 1 {
            'xloop: for x in bbox.0.x - 1..=bbox.1.x + 1 {
                for rect_coord in &rectangle {
                    if rect_coord.x == x && rect_coord.y == y {
                        print!("@");
                        continue 'xloop;
                    }
                }

                for coord in &self.coords {
                    if coord.x == x && coord.y == y {
                        print!("#");
                        continue 'xloop;
                    }
                }

                print!(".");
            }
            println!();
        }
    }

    pub fn is_in_shape(&self, y: u64) -> bool {
        let mut counter: u32 = 0;
        for i in 0..self.coords.len() {
            let y1 = self.coords[i].y;
            let y2 = self.coords[(i + 1) % self.coords.len()].y;

            let y_max = y1.max(y2);
            let y_min = y1.min(y2);

            if y_min == y_max {
                continue;
            }

            if y_min < y && y < y_max {
                counter += 1;
            }
        }

        counter % 2 == 1
    }
}

impl Display for Grid<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bbox = self.bounding_box().unwrap();
        for y in bbox.0.y - 1..=bbox.1.y + 1 {
            for x in bbox.0.x - 1..=bbox.1.x + 1 {
                let mut found = false;
                for coord in &self.coords {
                    if coord.x == x && coord.y == y {
                        write!(f, "#")?;
                        found = true;
                        break;
                    }
                }
                if !found {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: Ord + Copy> Grid<T> {
    pub fn bounding_box(&self) -> Option<(Coords<T>, Coords<T>)> {
        if self.coords.is_empty() {
            return None;
        }

        let mut min_x = self.coords[0].x;
        let mut max_x = self.coords[0].x;
        let mut min_y = self.coords[0].y;
        let mut max_y = self.coords[0].y;

        for coord in &self.coords {
            if coord.x < min_x {
                min_x = coord.x;
            }
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
        }

        Some((Coords { x: min_x, y: min_y }, Coords { x: max_x, y: max_y }))
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy> Coords<T> {
    pub fn rectangle(p1: &Coords<T>, p2: &Coords<T>) -> Vec<Coords<T>> {
        let c1 = Coords { x: p1.x, y: p1.y };

        let c2 = Coords { x: p1.x, y: p2.y };

        let c3 = Coords { x: p2.x, y: p1.y };

        let c4 = Coords { x: p2.x, y: p2.y };

        vec![c1, c2, c3, c4]
    }
}
