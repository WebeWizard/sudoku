#![allow(unused_parens)]
#![allow(unused_must_use)]

// import std
use std::io::Read;

// declare external crates
extern crate text_io;
extern crate time;
use time::precise_time_ns;

// import other project files
mod sudoku;
use sudoku::{Game, new_game};

// TODO: CHECK ROWS AND COLUMNS FOR NAKED AND HIDDEN SINGLES
// DONE?: Just going to assume that checking for hidden singles in subgrids is working as intended
// DONE?: Finish removing val_poss if Naked Locked subset
// TODO: Need to check for hidden singletons, triples, quads, omfg this his harder than I thought
// TODO: Maybe we should have generic technique functions that take a slice, instead of having
//           one function for subgrid and one for row and one for column
// DONE?: check for naked pairs, triples
// TODO: Re-organize project to better maintain multiple techniques

fn main() {
    let n: usize = 3;
    let size: usize = n.pow(2);

    // read in a bunch of puzzles
    let mut file = std::fs::File::open("./p096_sudoku.txt").unwrap();
    //let mut file = std::fs::File::open("./test.txt").unwrap();
    let mut file_buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let mut boards = Vec::new();
    for g in 0..(file_buf.len()/98)+1 {
        let mut board = Vec::new();
        for row in 0..size {
            let mut new_row:  Vec<usize> = Vec::new();
            for col in 0..size {
                let val = (file_buf[98*g+7+row*(size+1)+col+1] as char).to_digit(10).unwrap() as usize;
                new_row.push(val);
            }
            board.push(new_row);
        }
        boards.push(board);
    }

    // attempt to solve each board
    //for g in 2..3 {
    for g in 0..boards.len() {
        // parse each board into a Game
        let mut state: Game = new_game( n );
        let mut total_found: usize = 0;

        let start_time = precise_time_ns();

        // add the initial clues
        for row in 0..size {
            for col in 0..size {
                let val = boards[g][row][col];
                if ( val != 0) {
                    total_found += 1;
                    state.set_value( val, col, size-1-row );
                }
            }
        }

        // check each subgrid for discovered values, then set those values, repeat until done
        while ( total_found < size*size ) {
            let round_results = state.check_subgrids();
            for i in 0..round_results.len() {
                total_found += 1;
                state.set_value( round_results[i].0, round_results[i].1, round_results[i].2 );
            }
            println!("Game {}",g+1);
            println!("{}",state);
            if (round_results.len() == 0) {
                state.check_naked_subsets();
            }
        }

        let finish_time = precise_time_ns();
        println!("{}",state);
        println!("Time to solve: {}ns\n",finish_time-start_time);
    }

}
