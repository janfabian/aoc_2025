mod day01;
mod day02;
mod day03;
mod day04;
mod matrix;
mod read;

fn run_solution<S: read::Solution>(input: &str) {
    let solution = S::new(input);
    solution.execute();
}

fn main() {
    let file = read::read_file("./src/input/04_02.txt").unwrap();

    run_solution::<day04::PartB>(&file);
}
