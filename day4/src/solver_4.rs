pub fn solve(matrix: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                match check_angle(&matrix, (y as i32, x as i32)) {
                    Some(directions) => {
                        for direction in directions {
                            if follow_direction(&matrix, (y as i32, x as i32), direction) {
                                total += 1;
                            }
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
    (y_pos, x_pos): (i32, i32),
    (y_dir, x_dir): (i32, i32),
) -> bool {
    let word = "XMAS";
    let max_x = x_pos + (x_dir * 3);
    let max_y = y_pos + (y_dir * 3);
    if max_x as usize >= matrix[0].len() || max_y as usize >= matrix.len() || max_x < 0 || max_y < 0 {
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
fn check_angle(matrix: &Vec<Vec<char>>, (y_og, x_og): (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut directions: Vec<(i32, i32)> = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            if x_og + j >= 0
                && x_og + j < matrix[y_og as usize].len() as i32
                && y_og + i >= 0
                && y_og + i < matrix.len() as i32
            {
                if matrix[(y_og + i) as usize][(x_og + j) as usize] == 'M' {
                    directions.push((i, j));
                }
            } else {
                continue;
            }
        }
    }
    if directions.len() != 0 {
        return Some(directions);
    }
    return None;
}
