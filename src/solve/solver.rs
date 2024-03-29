use crate::common::grid_size::GRID_SIZE;
use crate::common::puzzle::Puzzle;
use crate::solve::coordinates::*;
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
        let iterations_limit = 100;
        let mut iterations_counter = 0;
        while
            iterations_counter < iterations_limit &&
            (self.set_obvious_ones() ||
                self.set_hiden_ones() ||
                self.notes_manager.set_obvious_pairs() ||
                self.notes_manager.use_square_methods())
        {
            iterations_counter += 1;
        }
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
        let mut result: bool = false;
        for value in 1..=GRID_SIZE {
            result |= perform_for_all_sets(|coordinates| { self.set_hidden(coordinates, value) });
        }
        result
    }

    fn set_hidden(&mut self, coordinates: &Coordinates, value: usize) -> bool {
        self.notes_manager.get_hidden(coordinates, value).map_or(false, |(row, col)| {
            self.set(row, col, value as u8);
            true
        })
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
