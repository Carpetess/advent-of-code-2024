use regex::Regex;
use std::collections::HashMap;
use std::fs;

mod solver;
mod sorter;


fn main() {
    let rule_regex = Regex::new(r"^\d+\|\d+$").unwrap();
    let print_regex = Regex::new(r"^\d+(,\d+)*$").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut prints: Vec<Vec<i32>> = Vec::new();
    let iterator = input.lines();
    for line in iterator {
        if rule_regex.is_match(line) {
            match parse_rule(line) {
                Some(rule) => match rules.get_mut(&rule.0) {
                    Some(current_rule_set) => {
                        current_rule_set.push(rule.1);
                    }
                    None => {
                        rules.insert(rule.0, Vec::new());
                        rules.get_mut(&rule.0).unwrap().push(rule.1);
                    }
                },
                None => println!("Parsing rule failed"),
            }
        } else if print_regex.is_match(line) {
            match parse_print(line) {
                Some(print) => prints.push(print),
                None => println!("Parsing print failed"),
            }
        }
    }

    println!("Total already in right order: {}", solver::solve(&rules, &mut prints));
    println!("Total in new order: {}", sorter::sort_result(&rules, &prints));
}

fn parse_print(line: &str) -> Option<Vec<i32>> {
    let mut result = Vec::new();
    for num in line.split(",") {
        if let Ok(integer) = num.parse::<i32>() {
            result.push(integer)
        } else {
            return None;
        }
    }
    return Some(result);
}

fn parse_rule(line: &str) -> Option<(i32, i32)> {
    let mut parts = line.split("|");
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();

    if let (Ok(x), Ok(y)) = (first.parse::<i32>(), second.parse::<i32>()) {
        Some((x, y))
    } else {
        None
    }
}
