mod day01;
mod read;

fn run_solution<S: read::Solution>(input: &str) {
    let solution = S::new(input);
    solution.execute();
}

fn main() {
    let file = read::read_file("./src/input/01_01.txt").unwrap();

    run_solution::<day01::Part1A>(&file);
    run_solution::<day01::Part1B>(&file);
}
