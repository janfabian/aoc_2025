mod day01;
mod day02;
mod day03;
mod read;

fn run_solution<S: read::Solution>(input: &str) {
    let solution = S::new(input);
    solution.execute();
}

fn main() {
    let file = read::read_file("./src/input/03_02.txt").unwrap();

    run_solution::<day03::PartB>(&file);
}
