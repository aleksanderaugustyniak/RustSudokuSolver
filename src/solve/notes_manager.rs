use crate::common::grid_size::GRID_SIZE;
use crate::common::puzzle::*;
use crate::solve::notes::Notes;

type HelpNotes = [u16; GRID_SIZE];
#[allow(clippy::unusual_byte_groupings)]
const FILLED_BITSET: u16 = 0b111_111_111; // Group by 3 fits better to the project purpose

pub struct NotesManager {
    puzzle: Puzzle,
    notes: Notes,
    row_notes: HelpNotes,
    col_notes: HelpNotes,
    square_notes: HelpNotes,
}

impl NotesManager {
    pub fn new(play_board: Puzzle) -> Self {
        let notes: Notes = Default::default();
        NotesManager {
            puzzle: play_board,
            notes,
            row_notes: Default::default(),
            col_notes: Default::default(),
            square_notes: Default::default(),
        }
    }

    pub fn fill(&mut self) {
        self.reset();
        for (row, notes_row) in self.notes.iter_mut().enumerate() {
            for (col, note) in notes_row.iter_mut().enumerate() {
                *note = if self.puzzle[row][col] != 0 { 0 } else { FILLED_BITSET };
            }
        }
        for i in 0..GRID_SIZE {
            self.set_row_based_notes(i);
            self.set_column_based_notes(i);
        }
        self.set_squares_based_notes();
    }

    pub fn get(&self) -> Notes {
        self.notes
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
        for row in self.notes.iter_mut() {
            row[col] &= self.col_notes[col];
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

    fn reset(&mut self) {
        for i in 0..GRID_SIZE {
            self.row_notes[i] = FILLED_BITSET;
            self.col_notes[i] = FILLED_BITSET;
            self.square_notes[i] = FILLED_BITSET;
        }
    }

    pub fn adjust(&mut self, row: usize, col: usize, value: u8) {
        for cell in self.notes[row].iter_mut() {
            Self::unset_note(cell, value);
        }
        Self::unset_note(&mut self.row_notes[row], value);

        for row_pos in self.notes.iter_mut() {
            Self::unset_note(&mut row_pos[col], value);
        }
        Self::unset_note(&mut self.col_notes[col], value);

        for x in Self::square_iter(row) {
            for y in Self::square_iter(col) {
                Self::unset_note(&mut self.notes[x][y], value);
            }
        }
        let square_index = Self::get_square_index(row, col);
        Self::unset_note(&mut self.square_notes[square_index], value);

        self.notes[row][col] = 0;
    }

    //TODO: extract bitset struct
    fn unset_note(note: &mut u16, bit: u8) {
        *note &= !(1 << (bit - 1));
    }

    fn is_set(note: &u16, position: usize) -> bool {
        let mask = 1 << (position - 1);
        (*note & mask) != 0
    }

    fn get_square_index(row: usize, col: usize) -> usize {
        3 * (row / 3) + col / 3
    }

    pub fn set_obvious_pairs(&mut self) -> bool {
        let mut any_progress = false;
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.notes[row][col].count_ones() == 2 {
                    let pair_note = self.notes[row][col];
                    any_progress |= self.set_coresponding_note(row, col, pair_note);
                }
            }
        }
        any_progress
    }

    fn set_coresponding_note(&mut self, x: usize, y: usize, note: u16) -> bool {
        let mut any_progress = false;
        if x < GRID_SIZE - 1 {
            for row in x + 1..GRID_SIZE {
                if self.notes[row][y] == note {
                    for row_to_clear in self.notes.iter_mut() {
                        if row_to_clear[y] != note {
                            row_to_clear[y] &= !note;
                            any_progress = true;
                        }
                    }
                }
            }
        }
        if y != GRID_SIZE - 1 {
            for col in y + 1..GRID_SIZE {
                if self.notes[x][col] == note {
                    for note_to_clear in self.notes[x].iter_mut() {
                        if *note_to_clear != note {
                            *note_to_clear &= !note;
                            any_progress = true;
                        }
                    }
                }
            }
        }
        any_progress
    }

    pub fn get_hidden_in_row(&mut self, row: usize, value: usize) -> Option<usize> {
        if !Self::is_set(&self.row_notes[row], value) {
            return None;
        }
        let mut count_values = 0;
        let mut col_found = 0;
        for (col, cell_note) in self.notes[row].iter().enumerate() {
            if Self::is_set(cell_note, value) {
                count_values += 1;
                col_found = col;
            }
        }
        if count_values == 1 {
            Some(col_found)
        } else {
            None
        }
    }

    pub fn get_hidden_in_col(&mut self, col: usize, value: usize) -> Option<usize> {
        if !Self::is_set(&self.col_notes[col], value) {
            return None;
        }
        let mut count_values = 0;
        let mut row_found = 0;
        for (row, row_cells) in self.notes.iter().enumerate() {
            if Self::is_set(&row_cells[col], value) {
                count_values += 1;
                row_found = row;
            }
        }
        if count_values == 1 {
            Some(row_found)
        } else {
            None
        }
    }

    pub fn get_hidden_in_square(&mut self, index: usize, value: usize) -> Option<(usize, usize)> {
        let mut count_values = 0;
        let mut row_found = 0;
        let mut col_found = 0;
        for row in Self::square_iter(index % 3) {
            for col in Self::square_iter(index / 3) {
                if Self::is_set(&self.notes[row][col], value) {
                    count_values += 1;
                    row_found = row;
                    col_found = col;
                }
            }
        }
        if count_values == 1 {
            Some((row_found, col_found))
        } else {
            None
        }
    }

    fn square_iter(position: usize) -> [usize; 3] {
        let start = position - (position % 3);
        [start, start + 1, start + 2]
    }

    pub fn use_square_methods(&mut self) -> bool {
        crate::solve::pointing_sets::Handler::new(&mut self.notes).handle()
    }
}

#[cfg(test)]
mod tests {
    include!("ut/test_notes_manager.rs");
}
