mod bigint;
mod coords;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
mod day09;
mod day11;
mod interval;
mod matrix;
mod op;
mod read;

fn run_solution<S: read::Solution>(input: &str) {
    let solution = S::new(input);
    solution.execute();
}

fn main() {
    let file = read::read_file("./src/input/11_02.txt").unwrap();

    run_solution::<day11::PartB>(&file);
}
