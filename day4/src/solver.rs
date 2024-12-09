pub fn solve(matrix: Vec<Vec<char>>) -> u32 {

    let mut total = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                match check_angle(&matrix, (x as i32, y as i32)) {
                    Some((x_dir, y_dir)) => {
                        if follow_direction(&matrix, (x as i32, y as i32), (x_dir, y_dir)) {
                            total += 1;
                        }
                    }
                    None => (),
                }
            }
        }
    }

    return total;
}
fn follow_direction(
    matrix: &Vec<Vec<char>>,
    (x_pos, y_pos): (i32, i32),
    (x_dir, y_dir): (i32, i32),
) -> bool {
    let word = "XMAS";
    let max_x = x_pos + (x_dir * 4);
    let max_y = y_pos + (y_dir * 4);
    if max_x as usize >= matrix.len() || max_y as usize >= matrix[0].len() {
        return false;
    }
    let mut valid = true;
    for (n_letter, letter) in word.chars().enumerate() {
        if valid {
            let (x_curr, y_curr) = (
                x_pos + (x_dir * n_letter as i32),
                y_pos + (y_dir * n_letter as i32),
            );
            valid = letter == matrix[y_curr as usize][x_curr as usize];
        }
    }
    return valid;
}
fn check_angle(matrix: &Vec<Vec<char>>, (x_og, y_og): (i32, i32)) -> Option<(i32, i32)> {
    println!(" NEW: {:?}, {}", (x_og, y_og), matrix[y_og as usize][x_og as usize] );

    let x_min = if x_og == 0 { x_og } else { x_og - 1 };
    let y_min = if y_og == 0 { y_og } else { y_og - 1 };
    let x_max = if x_og == matrix[0].len() as i32 - 1 {
        x_og
    } else {
        x_og + 1
    };

    let y_max = if y_og == matrix.len() as i32 - 1 {
        y_og
    } else {
        y_og + 1
    };
    println!("{}", y_max);
    println!("{}", y_min);
    for x in x_min..x_max {
        for y in y_min..y_max {
            if matrix[y as usize][x as usize] == 'M' {
                println!("{:?}", (x - x_og, y - y_og));
                return Some((x - x_og, y - y_og));
            }
        }
    }
    return None;
}
