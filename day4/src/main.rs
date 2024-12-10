mod solver_4;
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
    if temp != Vec::new() {
        matrix.push(temp);
    }
    println!("{:?}", solver_4::solve(matrix));
}
