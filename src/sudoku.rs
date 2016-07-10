use std::fmt;

// Game represents the entire state of the Game
// where 'board' is the current state of the grid
#[derive(Debug, Clone)]
pub struct Game {
    pub n: usize,
    pub size: usize,
    board: Vec<Vec<Space>>
}

// print Game in a human friendly form
impl fmt::Display for Game{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "~~~ Solved Sudoku ~~~");
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "{} ", self.board[x][self.size-1-y].value);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

// Space represents a position on the grid
// where 'value' is it's current value
// and 'posValSet' is the set of possible numbers that 'value' could be
#[derive(Debug, Clone)]
pub struct Space {
    value: usize,
    val_poss_set: Vec<usize>,
}

impl Game {

    pub fn set_value( &mut self, val: usize, column: usize, row: usize ) {
        // set the value of the space
        self.board[column][row].value = val;
        // remove the possibility for that space to hold any other values
        for i in 0..self.size {
            self.board[column][row].val_poss_set[i] = 0;
        }
        // update probabilities of partner spaces
        self.update_prob( val, column, row );
    }

    fn update_prob( &mut self, val: usize, column: usize, row: usize ) {
        // set probability of partners for value to 0
        // - row partners
        for i in 0..self.size {
            self.board[column][i].val_poss_set[val-1] = 0;
        }
        // - column partners
        for i in 0..self.size {
            self.board[i][row].val_poss_set[val-1] = 0;
        }
        // - subgrid partners
        let start_col = self.n*(column/self.n); // given 7, 7/3 = 2 and change, 2*3 = 6, profit
        let start_row = self.n*(row/self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                self.board[start_col+x][start_row+y].val_poss_set[val-1] = 0;
            }
        }

    }

    pub fn check_subgrids( &self ) -> Vec<(usize,usize,usize)> {
        let mut new_value_positions: Vec<(usize,usize,usize)> = Vec::new();
        for val in 1..self.size+1 {
            let subgrid_positions = self.check_subgrids_value( val );
            for i in 0..subgrid_positions.len() {
                new_value_positions.push( (val, subgrid_positions[i].0, subgrid_positions[i].1) );
            }
        }
        return new_value_positions;
    }

    fn check_subgrids_value( &self, val: usize ) -> Vec<(usize,usize)> {
        let mut new_value_positions: Vec<(usize,usize)> = Vec::new();
        for column in 0..self.n {
            for row in 0..self.n {
                let result = self.check_subgrid_value(val, column*self.n, row*self.n);
                if (  result != (self.size,self.size) ) {
                    new_value_positions.push( result );
                }
            }
        }
        return new_value_positions;
    }

    // checks a subgrid to see if there is only one position for a value
    // - returns (column,row) tuple of position
    // - if column and row = size then there was more than one open position, or if the value is already in the subgrid
    fn check_subgrid_value( &self, val: usize, column: usize, row: usize ) -> (usize,usize) {
        let start_col = self.n*(column/self.n);
        let start_row = self.n*(row/self.n);
        let mut non_zero_pos: Vec<(usize,usize)> = Vec::new();
        for x in 0..self.n {
            for y in 0..self.n {
                if ( self.board[start_col+x][start_row+y].value == val ) { return (self.size,self.size); }
                if ( self.board[start_col+x][start_row+y].val_poss_set[val-1] != 0 ) {
                    non_zero_pos.push((start_col+x,start_row+y));
                }
            }
        }
        if ( non_zero_pos.len() == 1 ) { return non_zero_pos[0]; }
        else {
            if ( non_zero_pos.len() > 1 ) {
                for i in 0..non_zero_pos.len() {
                    let mut count = 0;
                    for v in 0..self.size {
                        if ( self.board[non_zero_pos[i].0][non_zero_pos[i].1].val_poss_set[v] == 1 ) {
                            count += 1;
                        }
                    }
                    if ( count == 1 ) {
                        return non_zero_pos[i];
                    }
                }
            }
            return (self.size,self.size);
        }
    }

}

pub fn new_game( n: usize ) -> Game {
    let size = n.pow(2);
    // initial set of numbers
    let mut initial_set = Vec::new();
    for _ in 0..size {
        initial_set.push( 1 );
    }
    // initial space
    let initial_space = Space{ value: 0, val_poss_set: initial_set.clone() };
    // initial row
    let initial_row = vec![ initial_space; size];

    // create the board
    let board = vec![initial_row; size];
    // set up the game
    return Game{ n: n, size: size, board: board };
}
