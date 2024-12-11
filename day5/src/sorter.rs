use std::collections::HashMap;
use crate::solver;
use crate::solver::check_prev;

pub fn sort_result(rule_set :&HashMap<i32, Vec<i32>>, prints: &Vec<Vec<i32>>) -> i32 {
    let mut sorted_prints = prints.clone();
    for print in sorted_prints.iter_mut() {
        sort(rule_set, print, 0);
    }

    let mut total = 0;
    for print in sorted_prints.iter(){
        total += print[(print.len() - 1)/2];
    }
    return total;
}

fn sort(rule_set :&HashMap<i32, Vec<i32>>, print: &mut Vec<i32>, pos: usize) {
    for (c_pos, num) in print.iter().enumerate(){
        if c_pos >= pos {
            match check_prev(rule_set, print, num) {
                Some(i) => {
                    let temp = print[i];
                    print[i] = *num;
                   print[c_pos] = temp;
                    sort(rule_set, print,i);
                    break;
                },
                None => (),
            }
        }
    }
}

