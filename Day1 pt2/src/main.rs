use std::collections::HashMap;
use std::fs;

fn main() {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    let inputs = fs::read_to_string("../input.txt").unwrap();

    for line in inputs.lines() {
        let mut line = line.split("   ");
        list1.push(line.next().unwrap().parse::<u32>().unwrap());
        list2.push(line.next().unwrap().parse::<u32>().unwrap());
    }

    let mut repetitions: HashMap<u32, u32> = HashMap::new();

    for num in list1 {
        if(!repetitions.contains_key(&num)) {
            repetitions.insert(num, 0);
        }
    }
    for num in list2 {
        match repetitions.get(&num) {
            None => {continue}
            Some(current) => { repetitions.insert(num, current + num); }
        }
    }

    let mut similarity_score = 0;

    for num in repetitions.values() {
        similarity_score += num;
    }
    println!("{}", similarity_score);
}
