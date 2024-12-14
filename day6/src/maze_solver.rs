pub mod maze_solver {


    const BEEN_AT_CHAR: char = ',';

    pub struct Guard {
        going: (i32, i32),
        pos: (usize, usize),
        been_at: i32,
        pub(crate) maze: Vec<Vec<char>>,
        avatar: char,
    }

    impl Guard {
        pub fn new(going: (i32, i32), pos: (usize, usize), maze: Vec<Vec<char>>, avatar: char) -> Self {
            Self{
                going,
                pos,
                been_at: 0,
                maze,
                avatar,
            }
        }
        pub fn get_pos(&self) -> (usize, usize){
            self.pos
        }
        pub fn get_going(&self) -> (i32, i32){
            self.going
        }

        pub fn get_been_at(&self) -> i32 {
            self.been_at
        }
        pub fn walk_forward(&mut self) -> bool {
            let mut new_pos = match calculate_new_pos(self.pos, self.going) {
                Some(pos) => pos,
                None => return false,
            };
            if self.is_end_maze(new_pos){
                self.been_at += 1;
                self.maze[self.pos.0][self.pos.1] = BEEN_AT_CHAR;
                return false
            }
            while self.is_obstacle(new_pos){
                self.turn_right();
                new_pos = match calculate_new_pos(self.pos, self.going) {
                    Some(pos) => pos,
                    None => return false,
                };
            }
            if self.maze[new_pos.0][new_pos.1] != BEEN_AT_CHAR {
                self.been_at += 1;
            }
            self.maze[new_pos.0][new_pos.1] = self.avatar;
            self.maze[self.pos.0][self.pos.1] = BEEN_AT_CHAR;
            self.pos = new_pos;
            true
        }
        fn turn_right(&mut self) {
            self.going = match self.going {
                (1, 0) => {
                    self.avatar = '<';
                    (0, -1)
                }
                (0, 1) => {
                    self.avatar = 'v';
                    (1, 0)
                }
                (-1, 0) => {
                    self.avatar = '>';
                    (0, 1)
                }
                (0, -1) => {
                    self.avatar = '^';
                    (-1, 0)
                }
                _ => (0, 0),
            }
        }
        fn is_end_maze(&self, new_pos: (usize, usize)) -> bool {
            new_pos.0 >= self.maze.len() || new_pos.1 >= self.maze[self.pos.0].len()
        }
        fn is_obstacle(&self, new_pos: (usize, usize)) -> bool {
            self.maze[new_pos.0][new_pos.1] == '#'
        }
    }
    fn calculate_new_pos(pos: (usize, usize), going: (i32, i32)) -> Option<(usize, usize)> {
        let new_col = match add_i32_usize(pos.0, going.0) {
            Some(col) => col,
            None => return None
        };
        let new_row = match add_i32_usize(pos.1, going.1){
            Some(row) => row,
            None => return None
        };
            return Some((new_col, new_row))
    }
    fn add_i32_usize(x: usize, y: i32) -> Option<usize> {
        if y.is_negative() {
            let abs_y = y.wrapping_abs() as usize;
            if abs_y <= x {
                Some(x - abs_y)
            } else {
                None
            }
        } else {
            x.checked_add(y as usize)
        }
    }

}
