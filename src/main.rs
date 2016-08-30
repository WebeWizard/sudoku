extern crate sudokulib;
use sudokulib::game::Game;
use sudokulib::solver::solve;
use sudokulib::checker::check;

fn main() {
    let mut game: Game = Game::new(3);

    game.init_from_str("200080300060070084030500209000105408000000000402706000301007040720040060004010003");
    solve(&mut game);
    let valid = check(&game);
    if valid == true {
        println!("Sudoku puzzle solved!");
    } else {
        println!("PUZZLE RESULT IS NOT VALID");
    }
    println!("{}",game);
}