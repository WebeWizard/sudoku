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
    val_poss_set: Vec<bool>,
}

impl Game {

    pub fn set_value( &mut self, val: usize, column: usize, row: usize ) {
        // set the value of the space
        self.board[column][row].value = val;
        // remove the possibility for that space to hold any other values
        for i in 0..self.size {
            self.board[column][row].val_poss_set[i] = false;
        }
        // update probabilities of partner spaces
        self.update_prob( val, column, row );
    }

    fn update_prob( &mut self, val: usize, column: usize, row: usize ) {
        // set probability of partners for value to 0
        // - row partners
        for i in 0..self.size {
            self.board[column][i].val_poss_set[val-1] = false;
        }
        // - column partners
        for i in 0..self.size {
            self.board[i][row].val_poss_set[val-1] = false;
        }
        // - subgrid partners
        let start_col = self.n*(column/self.n); // given 7, 7/3 = 2 and change, 2*3 = 6, profit
        let start_row = self.n*(row/self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                self.board[start_col+x][start_row+y].val_poss_set[val-1] = false;
            }
        }

    }

    pub fn check_naked_subsets( &self ) {
        // check subgrids
        for x in 0..self.n {
            for y in 0..self.n {
                self.check_subgrid_naked_subsets(x,y);
            }
        }
        // check columns

        // check rows
    }

    // fml this all wrong.  so so wrong.
    // well not completely wrong... it will work for naked pairs, but not all subsets
    // NOTE:  if we don't detect a subset "A,B,C" for A, then maybe we'll detect it when testing B?
    fn check_subgrid_naked_subsets( &self, x: usize, y: usize ) {
        // repeat for each cell in the subgrid
        for col in x*self.n..x*self.n+self.n {
            for row in y*self.n..y*self.n+self.n {
                if ( self.board[col][row].value != 0 ) { continue; } // we only want to search non solved cells
                let mut matching_cells: Vec<(usize,usize)> = Vec::new();
                matching_cells.push( (col, row) );

                let mut sample_count = 0; // sample count
                for i in 0..self.size {
                    if ( self.board[col][row].val_poss_set[i] == true ) {
                        sample_count += 1;
                    }
                }
                // check to see if multiple cells contain the same val_poss_set
                // compare each cell with every other cell
                for new_col in x*self.n..x*self.n+self.n {
                    for new_row in y*self.n..y*self.n+self.n {
                        // don't compare a cell with itself
                        if (new_col == col && new_row == row) { continue; }
                        // don't exact match the val_poss_set, instead...
                        // check if number in the test set is <= number in the sample set
                        let mut new_count = 0; // test count
                        for i in 0..self.size {
                            if ( self.board[new_col][new_row].val_poss_set[i] == true ) {
                                new_count += 1;
                            }
                        }
                        if (new_count <= sample_count) {
                            // if it is, make sure the numbers in the test set are also in the sample set
                            let mut good = true;
                            for i in 0..self.size {
                                if ( self.board[new_col][new_row].val_poss_set[i] == true
                                &&  self.board[col][row].val_poss_set[i] == false) {
                                    good = false;
                                }
                            }
                            if (good == true) {
                                matching_cells.push( (new_col,new_row) );
                            }
                        }
                        // TODO: can we also detect hidden subsets this way? not quiet...
                        // probably a way to combine both techniques, but I don't care right now
                    }
                }
                // it's a naked subset if the number of possible values is the same as the number of matching cells
                // count the number of possible values
                if ( sample_count == matching_cells.len() ) {
                    // now that we have a naked subset, wtf do we do with it?
                    println!("We found a naked subset!");// remove the val_poss from all other subgrid cells
                    for nonss_col in x*self.n..x*self.n+self.n {
                        for nonss_row in y*self.n..y*self.n+self.n {
                            // skip if current cell is in the matching_cells
                            if ( in_subset( &matching_cells, (nonss_col,nonss_row)) == false ) {
                                // remove the sample val_poss set
                                for i in 0..self.size {
                                    if ( self.board[col][row].val_poss_set[i] == true ) {
                                        self.board[nonss_col][nonss_row].val_poss_set[i] == false;
                                    }
                                }
                            }
                        }
                    }
                    // detect if it's a locked subet?
                    // are they all in the same column?
                    let mut locked = true;
                    for i in 1..matching_cells.len() {
                        if ( matching_cells[i].0 != col ) { locked = false; }
                    }
                    if (locked == true) {
                        println!("We found a locked subset in a column!");
                    }
                    // are they all in the same row?
                    locked = true;
                    for i in 1..matching_cells.len() {
                        if ( matching_cells[i].1 != row ) { locked = false; }
                    }
                    if (locked == true) {
                        println!("We found a locked subset in a row!");
                    }
                }
                //println!("{:?}",matching_cells);
            }
        }
    }

    // TODO: Since we have multiple techniques now, we should pass in the technique we want to check
    // as a parameter
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
                if ( self.board[start_col+x][start_row+y].val_poss_set[val-1] != false ) {
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
                        if ( self.board[non_zero_pos[i].0][non_zero_pos[i].1].val_poss_set[v] == true ) {
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

fn in_subset(subset: &Vec<(usize,usize)>, cell: (usize,usize)) -> bool {
    for i in 0..subset.len() {
        if ( subset[i] == cell ) {
            return true;
        }
    }
    return false;
}

pub fn new_game( n: usize ) -> Game {
    let size = n.pow(2);
    // initial set of numbers
    let mut initial_set = Vec::new();
    for _ in 0..size {
        initial_set.push( true );
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
