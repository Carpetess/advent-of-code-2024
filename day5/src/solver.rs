use std::collections::HashMap;

pub fn solve(rule_set :HashMap<i32, Vec<i32>>, prints: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for print in prints {
        let mut valid = true;
        for num in &print {
            if valid {
                valid = check_prev(&rule_set, &print, num);
            }
        }
        if valid{
            total += print[(print.len() - 1)/2];
        }
    }
    return total;
}

fn check_prev(rule_set: &HashMap<i32, Vec<i32>>, print: &Vec<i32>, current: &i32) -> bool{
    match rule_set.get(&current){
        Some(rules) => {
            for next in print.iter() {
                if next == current {
                    return true;
                }
                for rule in rules {
                    if next == rule {
                        return false;
                    }
                }
            }
        },
        None => return true,
    }
    return true;
}