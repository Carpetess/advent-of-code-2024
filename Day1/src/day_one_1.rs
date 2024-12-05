
use std::fs;
pub fn solve_day_1() {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    let inputs = fs::read_to_string("../input.txt").unwrap();
    for line in inputs.lines() {
        let mut line = line.split("   ");
        list1.push(line.next().unwrap().parse::<u32>().unwrap());
        list2.push(line.next().unwrap().parse::<u32>().unwrap());
    }
    let list1_sorted = merge_sort(list1);
    let list2_sorted = merge_sort(list2);

    println!(
        "Distance: {}",
        calculate_distance(&list1_sorted, &list2_sorted)
    );
}

fn calculate_distance(list1: &[u32], list2: &[u32]) -> u32 {
    let mut distance: u32 = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        if a > b {
            distance += a - b;
        } else {
            distance += b - a;
        }
    }
    return distance;
}

fn merge_sort(mut list: Vec<u32>) -> Vec<u32> {
    if list.len() <= 1 {
        return list;
    }
    let second_half = list.split_off(list.len() / 2);

    let a = merge_sort(list.clone());
    let b = merge_sort(second_half.clone());
    merge(a, b)
}

fn merge(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut c: Vec<u32> = Vec::new();

    let mut iter_a = a.into_iter().peekable();
    let mut iter_b = b.into_iter().peekable();

    while iter_a.peek().is_some() || iter_b.peek().is_some() {
        match (iter_a.peek(), iter_b.peek()) {
            (Some(a), Some(b)) => {
                if a > b {
                    c.push(iter_b.next().unwrap());
                } else {
                    c.push(iter_a.next().unwrap());
                }
            }
            (Some(_), None) => {
                c.push(iter_a.next().unwrap());
            }
            (None, Some(_)) => {
                c.push(iter_b.next().unwrap());
            }
            (None, None) => {
                break;
            }
        }
    }
    while let Some(a) = iter_a.next() {
        c.push(a);
    }
    while let Some(b) = iter_b.next() {
        c.push(b);
    }
    c
}
