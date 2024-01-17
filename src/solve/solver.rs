use crate::solve::labels::*;

pub type Possibilities = [[u16; GRID_SIZE]; GRID_SIZE]; // way simplier solution than bitset with capacity
type HelpPossibilities = [u16; GRID_SIZE];

pub struct Solver {
    board: Labels,
    possibilities: Possibilities,
    row_possibilities: HelpPossibilities,
    col_possibilities: HelpPossibilities,
    square_possibilities: HelpPossibilities,
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
        let mut row_possibilities: HelpPossibilities = Default::default();
        let mut col_possibilities: HelpPossibilities = Default::default();
        let mut square_possibilities: HelpPossibilities = Default::default();
        for i in 0..GRID_SIZE {
            row_possibilities[i] = 0b111_111_111;
            col_possibilities[i] = 0b111_111_111;
            square_possibilities[i] = 0b111_111_111;
        }
        Solver {
            board: play_board,
            possibilities,
            row_possibilities,
            col_possibilities,
            square_possibilities,
        }
    }

    pub fn fill_possibilities(&mut self) {
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
            Self::unset_possibility(&mut self.row_possibilities[row], *cell);
        }
        for possibility in self.possibilities[row].iter_mut() {
            *possibility &= self.row_possibilities[row];
        }
    }

    fn check_column(&mut self, col: usize) {
        for row in self.board.iter() {
            if row[col] == 0 {
                continue;
            }
            Self::unset_possibility(&mut self.col_possibilities[col], row[col]);
        }
        for row_pos in self.possibilities.iter_mut() {
            row_pos[col] &= self.col_possibilities[col];
        }
    }

    fn check_squares(&mut self) {
        for square_x in 0..3 {
            for square_y in 0..3 {
                self.unset_square_possibilities(square_x, square_y);
            }
        }
        for (row, pos_row) in self.possibilities.iter_mut().enumerate() {
            for (col, possibility) in pos_row.iter_mut().enumerate() {
                if self.board[row][col] != 0 {
                    continue;
                }
                *possibility &= self.square_possibilities[3 * (row / 3) + col / 3];
            }
        }
    }

    fn unset_square_possibilities(&mut self, x: usize, y: usize) {
        for row in 3 * x..3 * (x + 1) {
            for col in 3 * y..3 * (y + 1) {
                if self.board[row][col] == 0 {
                    continue;
                }
                Self::unset_possibility(
                    &mut self.square_possibilities[3 * x + y],
                    self.board[row][col]
                );
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
