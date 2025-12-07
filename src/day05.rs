use crate::{interval::Interval, read::Solution};

fn parse(input: &str) -> (Vec<Range>, Vec<u128>) {
    let mut lines = input.lines();

    let mut ranges: Vec<Range> = vec![];
    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }
        let range = Range::from(line);

        ranges.push(range);
    }

    let mut ids: Vec<u128> = vec![];

    for line in lines {
        let id = line.parse().unwrap();
        ids.push(id);
    }

    (ranges, ids)
}

type Range = Interval<u128>;

pub struct PartA {
    ranges: Vec<Range>,
    ids: Vec<u128>,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        let (ranges, ids) = parse(input);
        PartA { ranges, ids }
    }

    fn execute(&self) {
        let mut result = 0;
        for id in &self.ids {
            if self.ranges.iter().any(|r| r.is_in_inclusive(id)) {
                result += 1;
            }
        }

        println!("result {result}")
    }
}
pub struct PartB {
    ranges: Vec<Range>,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        let (ranges, _) = parse(input);
        PartB { ranges }
    }

    fn execute(&self) {
        let mut ranges = self.ranges.clone();
        'outer: loop {
            for i in 0..=ranges.len() - 1 {
                for j in (i + 1)..ranges.len() {
                    let mut join_ranges = ranges[i].join(&ranges[j]);

                    if join_ranges.len() == 2 {
                        continue;
                    }

                    if join_ranges.len() == 1 {
                        println!(
                            "orig {} and {} --> merged {}",
                            &ranges[i], &ranges[j], &join_ranges[0]
                        );
                        ranges[j] = join_ranges.pop().unwrap();
                        ranges.remove(i);
                        continue 'outer;
                    }
                }
            }

            break;
        }

        let mut result: u128 = 0;
        for range in ranges {
            println!("interval {range}");
            result += range.diff() + 1;
        }

        println!("final result {result}")
    }
}
