use std::collections::HashMap;

pub fn solve(rule_set :&HashMap<i32, Vec<i32>>, prints: &mut Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for print in prints.clone().iter() {
        let mut valid = true;
        for num in print {
            if valid {
                 valid = match check_prev(rule_set, print, num){
                    Some(_) => false,
                    None => true,
                };
            }
        }
        if valid{
            total += print[(print.len() - 1)/2];
            prints.retain(|x| x != print);
        }
    }
    return total;
}

pub fn check_prev(rule_set: &HashMap<i32, Vec<i32>>, print: &Vec<i32>, current: &i32) -> Option<usize>{
    match rule_set.get(&current){
        Some(rules) => {
            for (pos, next) in print.iter().enumerate() {
                if next == current {
                    return None;
                }
                for rule in rules {
                    if next == rule {
                        return Some(pos);
                    }
                }
            }
        },
        None => return None,
    }
    return None;
}