use crate::solve::puzzle::*;

pub type Notes = [[u16; GRID_SIZE]; GRID_SIZE]; // way simplier solution than bitset with capacity
type HelpNotes = [u16; GRID_SIZE];

pub struct Solver {
    puzzle: Puzzle,
    notes: Notes,
    row_notes: HelpNotes,
    col_notes: HelpNotes,
    square_notes: HelpNotes,
}

impl Solver {
    pub fn new(play_board: Puzzle) -> Self {
        let mut notes: Notes = Default::default();
        for (row, notes_row) in notes.iter_mut().enumerate() {
            for (col, note) in notes_row.iter_mut().enumerate() {
                if play_board[row][col] != 0 {
                    *note = 0;
                    continue;
                }
                *note = 0b111_111_111;
            }
        }
        let mut row_notes: HelpNotes = Default::default();
        let mut col_notes: HelpNotes = Default::default();
        let mut square_notes: HelpNotes = Default::default();
        for i in 0..GRID_SIZE {
            row_notes[i] = 0b111_111_111;
            col_notes[i] = 0b111_111_111;
            square_notes[i] = 0b111_111_111;
        }
        Solver {
            puzzle: play_board,
            notes,
            row_notes,
            col_notes,
            square_notes,
        }
    }

    pub fn fill_notes(&mut self) {
        for i in 0..GRID_SIZE {
            self.set_row_based_notes(i);
            self.set_column_based_notes(i);
        }
        self.set_squares_based_notes();
    }

    fn set_row_based_notes(&mut self, row: usize) {
        for cell in self.puzzle[row].iter() {
            if *cell == 0 {
                continue;
            }
            Self::unset_note(&mut self.row_notes[row], *cell);
        }
        for note in self.notes[row].iter_mut() {
            *note &= self.row_notes[row];
        }
    }

    fn set_column_based_notes(&mut self, col: usize) {
        for row in self.puzzle.iter() {
            if row[col] == 0 {
                continue;
            }
            Self::unset_note(&mut self.col_notes[col], row[col]);
        }
        for row_pos in self.notes.iter_mut() {
            row_pos[col] &= self.col_notes[col];
        }
    }

    fn set_squares_based_notes(&mut self) {
        for square_x in 0..3 {
            for square_y in 0..3 {
                self.set_square_notes(square_x, square_y);
            }
        }
        for (row, notes_row) in self.notes.iter_mut().enumerate() {
            for (col, note) in notes_row.iter_mut().enumerate() {
                if self.puzzle[row][col] != 0 {
                    continue;
                }
                *note &= self.square_notes[Self::get_square_index(row, col)];
            }
        }
    }

    fn set_square_notes(&mut self, x: usize, y: usize) {
        for row in 3 * x..3 * (x + 1) {
            for col in 3 * y..3 * (y + 1) {
                if self.puzzle[row][col] == 0 {
                    continue;
                }
                Self::unset_note(&mut self.square_notes[3 * x + y], self.puzzle[row][col]);
            }
        }
    }

    pub fn get_notes(&self) -> &Notes {
        &self.notes
    }

    pub fn get_solution(&self) -> Puzzle {
        self.puzzle
    }

    fn unset_note(note: &mut u16, bit: u8) {
        *note &= !(1 << (bit - 1));
    }

    pub fn solve(&mut self) {
        self.fill_notes();
        while self.set_obvious_ones() {}
    }

    fn set_obvious_ones(&mut self) -> bool {
        let mut any_cell_filled: bool = false;
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.notes[row][col].count_ones() == 1 {
                    let value = (self.notes[row][col].trailing_zeros() + 1) as u8;
                    self.set(row, col, value);
                    any_cell_filled = true;
                }
            }
        }
        any_cell_filled
    }

    fn set(&mut self, row: usize, col: usize, value: u8) {
        self.puzzle[row][col] = value;
        self.adjust_notes(row, col, value);
    }

    fn adjust_notes(&mut self, row: usize, col: usize, value: u8) {
        for cell in self.notes[row].iter_mut() {
            Self::unset_note(cell, value);
        }
        Self::unset_note(&mut self.row_notes[row], value);

        for row_pos in self.notes.iter_mut() {
            Self::unset_note(&mut row_pos[col], value);
        }
        Self::unset_note(&mut self.col_notes[col], value);

        let square_index = Self::get_square_index(row, col);
        for x in 3 * (square_index / 3)..3 * (square_index / 3 + 1) {
            for y in 3 * (square_index % 3)..3 * ((square_index % 3) + 1) {
                Self::unset_note(&mut self.notes[x][y], value);
            }
        }
        Self::unset_note(&mut self.square_notes[square_index], value);

        self.notes[row][col] = 0;
    }

    fn get_square_index(row: usize, col: usize) -> usize {
        3 * (row / 3) + col / 3
    }
}

#[cfg(test)]
mod tests {
    include!("ut/test_solver.rs");
}
