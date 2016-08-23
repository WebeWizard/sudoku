extern crate sudokulib;
use sudokulib::game::Game;
use sudokulib::solver::solve;

fn main() {
    let mut game: Game = Game::new(3);

    game.init_from_str("280000473534827196071034080300500040000340060460790310090203654003009821000080937");

    solve(&mut game);

    println!("{}",game);
}