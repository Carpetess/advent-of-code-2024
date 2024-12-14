mod maze_solver;
use crate::maze_solver::maze_solver::*;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found: input.txt");
    let mut maze: Vec<Vec<char>> = Vec::new();

    for line in input.split("\n") {
        let mut temp: Vec<char> = Vec::new();
        for char in line.chars() {
            temp.push(char)
        }
        maze.push(temp);
    }
    let mut pos = (0,0);

    for (y, line) in maze.iter().enumerate() {
        for (x, char) in line.iter().enumerate(){
            if *char == '^'{
                pos = (y,x);
            }
        }
    }

    let mut guard: Guard = Guard::new((-1,0), pos, maze, '^');

    while guard.walk_forward(){
        for  line in &guard.maze{
            for char in line{
                print!("{}", char);
            }
            println!()
        }
        println!("POS: {:?}", &guard.get_pos());
        println!("LOOKING AT: {:?}", &guard.get_going());

    }
    println!("{}", &guard.get_been_at()+1)
}
