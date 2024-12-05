use std::collections::HashMap;
use std::fs;

fn main() {
    let mut repetitions: HashMap<u32, u32> = HashMap::new();
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    let inputs = fs::read_to_string("../input.txt").unwrap();
    for line in inputs.lines() {
        let mut line = line.split("   ");
        list1.push(line.next().unwrap().parse::<u32>().unwrap());
        list2.push(line.next().unwrap().parse::<u32>().unwrap());
    }
}
