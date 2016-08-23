extern crate sudokulib;
use sudokulib::game::Game;
use sudokulib::solver::solve;

fn main() {
    let mut game: Game = Game::new(3);

    game.init_from_str("100920000524010000000000070050008102000000000402700090060000000000030945000071006");
    solve(&mut game);

    println!("{}",game);
}