use std::fs;

mod solver;

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    let mut list_of_reports = Vec::new();

    for line in input.lines() {
        let mut report = Vec::new();
        for number in line.split_ascii_whitespace() {
            report.push(number.parse::<u32>().unwrap());
        }
        list_of_reports.push(report);
    }

    println!("{}", solver::solve_1(list_of_reports));

}
