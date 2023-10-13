mod solver;
mod utils;

fn main() {
    let game = utils::Game::DEFAULT;
    println!("Board is valid: {}", game.is_valid());
    solver::solve(game);
}
