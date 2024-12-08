use std::fs;

mod solver;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    println!("{}", solver::solve(&input));
}
