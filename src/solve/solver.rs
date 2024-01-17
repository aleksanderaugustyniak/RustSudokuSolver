use crate::solve::labels::*;

pub type Possibilities = [[u16; GRID_SIZE]; GRID_SIZE]; // way simplier solution than bitset with capacity 9

pub struct Solver {
    board: Labels,
    possibilities: Possibilities,
}

impl Solver {
    pub fn new(play_board: Labels) -> Self {
        let mut possibilities: Possibilities = Default::default();
        for (row, pos_row) in possibilities.iter_mut().enumerate() {
            for (col, possibility) in pos_row.iter_mut().enumerate() {
                if play_board[row][col] != 0 {
                    *possibility = 0;
                    continue;
                }
                *possibility = 0b111_111_111;
            }
        }
        Solver {
            board: play_board,
            possibilities,
        }
    }

    pub fn solve(&mut self) {
        //-> Labels {
        for i in 0..GRID_SIZE {
            self.check_row(i);
            self.check_column(i);
        }
        self.check_squares();
    }

    fn check_row(&mut self, row: usize) {
        for cell in self.board[row].iter() {
            if *cell == 0 {
                continue;
            }
            for possibility in self.possibilities[row].iter_mut() {
                Self::unset_possibility(possibility, *cell);
            }
        }
    }

    fn check_column(&mut self, col: usize) {
        for row in self.board.iter() {
            if row[col] == 0 {
                continue;
            }
            for row_pos in self.possibilities.iter_mut() {
                Self::unset_possibility(&mut row_pos[col], row[col]);
            }
        }
    }

    fn check_squares(&mut self) {
        for square_x in 0..3 {
            for square_y in 0..3 {
                self.check_square(square_x, square_y);
            }
        }
    }

    fn check_square(&mut self, x: usize, y: usize) {
        for row in 3 * x..3 * (x + 1) {
            for col in 3 * y..3 * (y + 1) {
                if self.board[row][col] != 0 {
                    for row_pos in 3 * x..3 * (x + 1) {
                        for col_pos in 3 * y..3 * (y + 1) {
                            Self::unset_possibility(
                                &mut self.possibilities[row_pos][col_pos],
                                self.board[row][col]
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn get_possibilities(&self) -> &Possibilities {
        &self.possibilities
    }

    fn unset_possibility(possibility: &mut u16, bit_index: u8) {
        *possibility &= !(1 << (bit_index - 1));
    }
}

#[cfg(test)]
mod tests {
    include!("ut/test_solver.rs");
}
