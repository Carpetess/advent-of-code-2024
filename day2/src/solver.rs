
pub fn solve(list_of_reports: &Vec<Vec<u32>>, dampener: bool) -> u32 {
    let mut num_of_safe_reports: u32 = 0;
    for report in list_of_reports {
        if check_ascending(report, dampener) || check_descending(report, dampener) {
            num_of_safe_reports += 1;
        }
    }
    num_of_safe_reports
}
fn check_descending(report: &Vec<u32>, dampener: bool) -> bool {
    let mut failed = false;
    let mut max = u32::MAX;
    for num in report.iter() {
        if (*num >= max || (num + 4) <= max) && max != u32::MAX {
            failed = true; break;
        }
        max = *num;
    }
    if dampener && failed{
        let mut i = 0;
        while i < report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if check_descending(&new_report, false){
                return true;
            }
            i+=1;
        }
    }
    !failed
}
fn check_ascending(report: &Vec<u32>, dampener: bool) -> bool {
    let mut failed = false;
    let mut min = u32::MIN;
    for num in report.iter() {
        if (*num <= min || (min + 4) <= *num) && min != u32::MIN {
            failed = true; break;
        }
        min = *num;
    }
    if dampener && failed{
        let mut i = 0;
        while i < report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if check_ascending(&new_report, false){
                return true;
            }
            i+=1;
        }
    }
    !failed
}
