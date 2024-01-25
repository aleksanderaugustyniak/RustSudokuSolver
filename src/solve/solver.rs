use crate::common::grid_size::GRID_SIZE;
use crate::common::puzzle::*;
use crate::solve::notes_manager::NotesManager;

pub struct Solver {
    puzzle: Puzzle,
    notes_manager: NotesManager,
}

impl Solver {
    pub fn new(play_board: Puzzle) -> Self {
        let mut filler = NotesManager::new(play_board);
        filler.fill();
        Solver {
            puzzle: play_board,
            notes_manager: filler,
        }
    }

    pub fn get_solution(&self) -> Puzzle {
        self.puzzle
    }

    pub fn solve(&mut self) {
        while
            self.set_obvious_ones() ||
            self.set_hiden_ones() ||
            self.notes_manager.set_obvious_pairs()
        {}
    }

    fn set_obvious_ones(&mut self) -> bool {
        let mut any_cell_filled: bool = false;
        let notes = self.notes_manager.get();
        for (row, notes_row) in notes.iter().enumerate() {
            for (col, note) in notes_row.iter().enumerate() {
                if note.count_ones() == 1 {
                    let value = (note.trailing_zeros() + 1) as u8;
                    self.set(row, col, value);
                    any_cell_filled = true;
                }
            }
        }
        any_cell_filled
    }

    fn set_hiden_ones(&mut self) -> bool {
        let mut any_cell_filled: bool = false;
        for index in 0..GRID_SIZE {
            for value in 1..=GRID_SIZE {
                any_cell_filled |= self.set_hidden_in_row(index, value);
                any_cell_filled |= self.set_hidden_in_col(index, value);
                any_cell_filled |= self.set_hidden_in_squares(index, value);
            }
        }
        any_cell_filled
    }

    fn set_hidden_in_row(&mut self, row: usize, value: usize) -> bool {
        match self.notes_manager.get_hidden_in_row(row, value) {
            Some(col) => {
                self.set(row, col, value as u8);
                true
            }
            None => { false }
        }
    }

    fn set_hidden_in_col(&mut self, col: usize, value: usize) -> bool {
        match self.notes_manager.get_hidden_in_col(col, value) {
            Some(row) => {
                self.set(row, col, value as u8);
                true
            }
            None => { false }
        }
    }

    fn set_hidden_in_squares(&mut self, index: usize, value: usize) -> bool {
        match self.notes_manager.get_hidden_in_square(index, value) {
            Some((row, col)) => {
                self.set(row, col, value as u8);
                true
            }
            None => { false }
        }
    }

    fn set(&mut self, row: usize, col: usize, value: u8) {
        self.puzzle[row][col] = value;
        self.notes_manager.adjust(row, col, value);
    }
}

#[cfg(test)]
mod tests {
    include!("ut/test_solver.rs");
}
