use crate::utils::Game;
use std::collections::HashSet;

pub fn solve(game: &mut Game) -> Vec<Game> {
    if game.check_win() {
        return vec![game.clone()];
    }
    if !game.is_valid() {
        return Vec::new();
    }
    let mut solutions = Vec::new();
    for i in 0..=8 {
        for j in 0..=8 {
            if game.board[i][j] != 0 {
                continue;
            }
            let n = j / 3 + (i / 3) * 3;
            let avaible = intersection_of_three(
                get_avaible_square(game.board, n),
                get_avaible_row(game.board, i),
                get_avaible_col(game.board, j),
            );

            for &num in avaible.iter() {
                game.board[i][j] = num;
                let res = solve(game);
                if res.len() > 0 {
                    solutions.extend(res);
                }
                game.board[i][j] = 0;
            }
            return solutions;
        }
    }
    solutions
}
pub fn get_avaible_square(board: [[u16; 9]; 9], n: usize) -> HashSet<u16> {
    let mut arr = Vec::new();

    let starting_row = ((n / 3) * 3) as usize;
    let starting_col = ((n % 3) * 3) as usize;

    for i in starting_row..=starting_row + 2 {
        for j in starting_col..=starting_col + 2 {
            if board[i][j] == 0 {
                continue;
            }
            arr.push(board[i][j])
        }
    }
    let set: HashSet<u16> = (1..10).collect();
    let i: HashSet<u16> = arr.into_iter().collect();
    set.difference(&i).cloned().collect()
}
pub fn get_avaible_row(board: [[u16; 9]; 9], row: usize) -> HashSet<u16> {
    let mut arr = Vec::new();

    for i in 0..8 {
        if board[row][i] == 0 {
            continue;
        }
        arr.push(board[row][i])
    }
    let set: HashSet<u16> = (1..10).collect();
    let i: HashSet<u16> = arr.into_iter().collect();

    set.difference(&i).cloned().collect()
}
pub fn get_avaible_col(board: [[u16; 9]; 9], col: usize) -> HashSet<u16> {
    let mut arr = Vec::new();

    for i in 0..8 {
        if board[i][col] == 0 {
            continue;
        }
        arr.push(board[i][col])
    }
    let set: HashSet<u16> = (1..10).collect();
    let i: HashSet<u16> = arr.into_iter().collect();

    set.difference(&i).cloned().collect()
}

fn intersection_of_three(h1: HashSet<u16>, h2: HashSet<u16>, h3: HashSet<u16>) -> HashSet<u16> {
    h1.intersection(&h2)
        .cloned()
        .collect::<HashSet<u16>>()
        .intersection(&h3)
        .cloned()
        .collect::<HashSet<u16>>()
}
