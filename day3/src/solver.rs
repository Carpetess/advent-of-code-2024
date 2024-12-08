pub fn solve(input: &String) -> i32 {
    let mut num = 0;
    let mut total = 0;
    for i in 0..input.len() {
        if input[i..].starts_with("mul(") {
            match check_fmt(input, i + 4) {
                Some(x) => { total += x;
                    num += 1;
                } ,
                None => {
                    continue;
                },
            }
        }
    }
    println!("{}", num);
    total
}

fn check_fmt(input: &String, start: usize) -> Option<i32> {
    let mut valid = true;
    let mut current_char_pos = start;
    let mut current_number: String = String::new();
    let mut holder: Vec<String> = Vec::new();
    while valid {
        let c = input.chars().nth(current_char_pos).unwrap();
        match c {
            ',' => {
                holder.push(current_number);
                current_number = String::new();
            }
            ')' => {
                holder.push(current_number);
                current_number = String::new();
                valid = false;
            }
            _ => {
                if c.is_numeric() {
                    current_number = current_number + c.to_string().as_str();
                } else {
                    return None;
                }

            }
        }
        current_char_pos += 1;
    }

    if holder.len() == 2 {
        let mut result = (0, 0);
        match holder[0].parse::<i32>() {
            Ok(num) => result.0 = num,
            Err(_) => return None,
        }
        match holder[1].parse::<i32>() {
            Ok(num) => result.1 = num,
            Err(_) => return None,
        }
        println!("{:?}", result);
        return Some(result.1 * result.0);
    }
    None
}
