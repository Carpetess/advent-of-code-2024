
    pub fn solve_1(list_of_reports: Vec<Vec<u32>>) -> u32{
        let mut num_of_safe_reports: u32 = 0;
        for report in list_of_reports{
            if check_ascending(&report) || check_descending(&report){
                num_of_safe_reports += 1;
                println!("{:?} PASSED", report);
            } else { println!("{:?} NOT PASSED", report);  }
        }
        num_of_safe_reports
    }
    fn check_descending(report: &Vec<u32>) -> bool{
        let mut max = u32::MAX;
        for num in report.iter() {
            if (*num >= max || (num + 4) <= max) && max != u32::MAX {
                return false;
            }
            max = *num;
        }
        true
    }
    fn check_ascending(report: &Vec<u32>) -> bool{
        let mut min = u32::MIN;
        for num in report.iter() {
            if (*num <= min || (min + 4) <= *num) && min != u32::MIN {
                return false;
            }
            min = *num;
        }

        true
    }