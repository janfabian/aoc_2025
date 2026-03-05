use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(PartialEq, Eq)]
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
        for y in bbox.0.y - (if bbox.0.y > 0 { 1 } else { 0 })..=bbox.1.y + 1 {
            'xloop: for x in bbox.0.x - (if bbox.0.x > 0 { 1 } else { 0 })..=bbox.1.x + 1 {
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

    pub fn is_in_shape(&self, p: &Coords<u64>) -> bool {
        let mut counter: u32 = 0;
        for i in 0..self.coords.len() {
            let p1 = &self.coords[i];
            let p2 = &self.coords[(i + 1) % self.coords.len()];

            if p1.x != p2.x {
                continue;
            }

            if p.x >= p1.x {
                continue;
            }

            let y_max = p1.y.max(p2.y);
            let y_min = p1.y.min(p2.y);

            if y_min < p.y && p.y < y_max {
                counter += 1;
            }
        }

        counter % 2 == 1
    }

    pub fn is_on_boundary(&self, p: &Coords<u64>) -> bool {
        for i in 0..self.coords.len() {
            let p1 = &self.coords[i];
            let p2 = &self.coords[(i + 1) % self.coords.len()];

            if (p.x == p1.x && p.x == p2.x && p.y <= p1.y.max(p2.y) && p.y >= p1.y.min(p2.y))
                || (p.y == p1.y && p.y == p2.y && p.x <= p1.x.max(p2.x) && p.x >= p1.x.min(p2.x))
            {
                return true;
            }
        }

        return false;
    }

    fn cross_h_v(h: &[&Coords<u64>; 2], v: &[&Coords<u64>; 2]) -> bool {
        let min_h_x = h[0].x.min(h[1].x);
        let max_h_x = h[0].x.max(h[1].x);

        let min_v_y = v[0].y.min(v[1].y);
        let max_v_y = v[0].y.max(v[1].y);

        let x_cond = min_h_x <= v[0].x && v[0].x <= max_h_x;
        let y_cond = min_v_y <= h[0].y && h[0].y <= max_v_y;

        return x_cond && y_cond;
    }

    pub fn cross_segment(&self, p: &[Coords<u64>]) -> bool {
        for i in 0..p.len() {
            let p1 = &p[i];
            let p2 = &p[(i + 1) % p.len()];

            if p1 == p2 {
                continue;
            }

            for j in 0..self.coords.len() {
                let q1 = &self.coords[j];
                let q2 = &self.coords[(j + 1) % self.coords.len()];

                if q1 == q2 {
                    continue;
                }

                let semi_res = match (p1.x == p2.x, p1.y == p2.y, q1.x == q2.x, q1.y == q2.y) {
                    (true, _, _, true) => Grid::cross_h_v(&[q1, q2], &[p1, p2]),
                    (_, true, true, _) => Grid::cross_h_v(&[p1, p2], &[q1, q2]),
                    _ => false,
                };

                if semi_res {
                    return true;
                }
            }
        }

        return false;
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
