mod solver;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut matrix = Vec::new();
    let mut temp = Vec::new();
    for char in input.chars() {
        if char == '\r' {
            matrix.push(temp);
            temp = Vec::new();
        } else if char != '\n' {
            temp.push(char);
        }
    }
    matrix.remove(matrix.len()-1);
    println!("{:?}", matrix);
    println!("{:?}", solver::solve(matrix));
}
