pub fn solve(matrix: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'M' || *c == 'S' {
                if check_either(&matrix, (y as i32,x as i32), (1,1)){
                    total += 1;
                }
            }
        }
    }
    return total;
}

fn check_either(matrix: &Vec<Vec<char>>, (y_og, x_og): (i32, i32), (dir_y, dir_x): (i32, i32)) -> bool {
    let max_x = x_og + (dir_x * 2);
    let max_y = y_og + (dir_y * 2);

    if max_x < 0 || max_y < 0 || max_x >= matrix[0].len() as i32 || max_y >= matrix.len() as i32 || x_og >= matrix[0].len() as i32 {
        return false;
    }

    if matrix[y_og as usize][x_og as usize] == 'M' {
        check_word(matrix, (y_og, x_og), (dir_y, dir_x)) && (check_word(matrix, (y_og, x_og + 2), (1, -1)))
    } else if matrix[y_og as usize][x_og as usize] == 'S' {
        check_word(matrix, (y_og, x_og), (dir_y, dir_x)) && check_word(matrix, (y_og, x_og + 2), (1, -1))
    } else {
        false
    }
}
fn check_word(matrix: &Vec<Vec<char>>, (y_og, x_og): (i32, i32), (dir_y, dir_x): (i32, i32)) -> bool {
    let word = if matrix[y_og as usize][x_og as usize] == 'M' {
        "MAS"
    } else {
        "SAM"
    };
    let mut valid = true;
    for (n_letter, letter) in word.chars().enumerate() {
        if valid{
            let x_curr = x_og + (n_letter as i32 * dir_x);
            let y_curr = y_og + (n_letter as i32 * dir_y);
            valid = letter == matrix[y_curr as usize][x_curr as usize];
        }
    }
    return valid;
}