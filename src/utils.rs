#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub board: [[u16; 9]; 9],
}

impl Game {
    pub fn new(board: [[u16; 9]; 9]) -> Game {
        Game { board }
    }

    pub const DEFAULT: Game = Game {
        board: [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ],
    };
    pub const OTHER: Game = Game {
        board: [
            [3, 0, 0, 8, 0, 1, 0, 0, 2],
            [2, 0, 1, 0, 3, 0, 6, 0, 4],
            [0, 0, 0, 2, 0, 4, 0, 0, 0],
            [8, 0, 9, 0, 0, 0, 1, 0, 6],
            [0, 6, 0, 0, 0, 0, 0, 5, 0],
            [7, 0, 2, 0, 0, 0, 4, 0, 9],
            [0, 0, 0, 5, 0, 0, 0, 0, 0],
            [9, 0, 4, 0, 0, 0, 0, 0, 5],
            [6, 0, 0, 1, 0, 7, 0, 0, 3],
        ],
    };

    pub fn check_win(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }
        return self.is_valid();
    }

    pub fn print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.board[i][j]);
            }
            println!();
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..9 {
            if has_duplicate(self.board[i]) {
                return false;
            }
        }
        for i in 0..9 {
            let mut arr = [0; 9];
            for j in 0..9 {
                arr[j] = self.board[j][i];
            }
            if has_duplicate(arr) {
                return false;
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                let mut arr = [0; 9];
                for k in 0..3 {
                    for l in 0..3 {
                        arr[k * 3 + l] = self.board[i * 3 + k][j * 3 + l];
                    }
                }
                if has_duplicate(arr) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn has_duplicate(arr: [u16; 9]) -> bool {
    let mut seen = 0;
    for i in arr.iter() {
        if i == &0 {
            continue;
        }
        if seen & (1 << i) > 0 {
            return true;
        }
        seen |= 1 << i;
    }
    return false;
}
