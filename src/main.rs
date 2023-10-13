mod solver;
mod utils;

fn main() {
    let mut game = utils::Game::OTHER;
    println!("Board is valid: {}", game.is_valid());
    let vec = solver::solve(&mut game);
    println!("Solutions: {}", vec.len());
    for i in vec.iter() {
        i.print();
        println!();
    }
}
