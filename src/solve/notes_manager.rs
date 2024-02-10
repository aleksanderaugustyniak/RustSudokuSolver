use crate::common::grid_size::GRID_SIZE;
use crate::common::puzzle::Puzzle;
use crate::solve::coordinates::*;
use crate::solve::notes::Notes;

#[allow(clippy::unusual_byte_groupings)]
const FILLED_BITSET: u16 = 0b111_111_111; // Group by 3 fits better to the project purpose

pub struct NotesManager {
    puzzle: Puzzle,
    notes: Notes,
}

impl NotesManager {
    pub fn new(play_board: Puzzle) -> Self {
        let notes: Notes = Default::default();
        NotesManager {
            puzzle: play_board,
            notes,
        }
    }

    pub fn fill(&mut self) {
        for (row, notes_row) in self.notes.iter_mut().enumerate() {
            for (col, note) in notes_row.iter_mut().enumerate() {
                *note = if self.puzzle[row][col] != 0 { 0 } else { FILLED_BITSET };
            }
        }
        for index in 0..GRID_SIZE {
            self.set_notes(&get_row_coordinates(index));
            self.set_notes(&get_col_coordinates(index));
            self.set_notes(&get_square_coordinates((index / 3, index % 3)));
        }
    }

    pub fn get(&self) -> Notes {
        self.notes
    }

    fn set_notes(&mut self, coordinates: &Coordinates) {
        let note = self.get_notes(coordinates);
        for (row, col) in coordinates.iter() {
            if self.puzzle[*row][*col] == 0 {
                self.notes[*row][*col] &= note;
            }
        }
    }

    fn get_notes(&mut self, coordinates: &Coordinates) -> u16 {
        let mut note: u16 = FILLED_BITSET;
        for (row, col) in coordinates.iter() {
            if self.puzzle[*row][*col] == 0 {
                continue;
            }
            Self::unset_note(&mut note, self.puzzle[*row][*col]);
        }
        note
    }

    pub fn adjust(&mut self, row: usize, col: usize, value: u8) {
        for cell in self.notes[row].iter_mut() {
            Self::unset_note(cell, value);
        }

        for row_pos in self.notes.iter_mut() {
            Self::unset_note(&mut row_pos[col], value);
        }

        for (x, y) in get_square_coordinates((row / 3, col / 3)) {
            Self::unset_note(&mut self.notes[x][y], value);
        }
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

    pub fn set_obvious_pairs(&mut self) -> bool {
        let mut any_progress = false;
        for index in 0..GRID_SIZE {
            any_progress |= self.perform_obvious_pair(&get_row_coordinates(index));
            any_progress |= self.perform_obvious_pair(&get_col_coordinates(index));
            any_progress |= self.perform_obvious_pair(
                &get_square_coordinates((index / 3, index % 3))
            );
        }
        any_progress
    }

    fn perform_obvious_pair(&mut self, coordinates: &Coordinates) -> bool {
        let mut result = false;
        for (row, col) in coordinates.iter() {
            if self.notes[*row][*col].count_ones() == 2 {
                result |= self.handle_coresponding_note(coordinates, (*row, *col));
            }
        }
        if result {
            return true;
        }
        for (index, (row, col)) in coordinates.iter().enumerate() {
            if self.notes[*row][*col].count_ones() <= 3 && self.notes[*row][*col] != 0 {
                self.handle_triple(coordinates, index, self.notes[*row][*col]);
            }
        }
        result
    }

    fn handle_triple(&mut self, coordinates: &Coordinates, index: usize, first_note: u16) {
        if index >= GRID_SIZE - 2 {
            return; // can't find triple on last or previous one
        }
        for second in index + 1..GRID_SIZE - 1 {
            let (row, col) = coordinates[second];
            let second_note = self.notes[row][col] | first_note;
            if second_note.count_ones() <= 3 && self.notes[row][col] != 0 {
                for third in second + 1..GRID_SIZE {
                    let (row3, col3) = coordinates[third];
                    let note = second_note | self.notes[row3][col3];
                    if note.count_ones() <= 3 && self.notes[row3][col3] != 0 {
                        self.clear_notes_obvious_set_based(coordinates, note);
                    }
                }
            }
        }
    }

    fn handle_coresponding_note(&mut self, coordinates: &Coordinates, (x, y): Point) -> bool {
        let mut any_progress = false;
        let note = self.notes[x][y];
        for (row, col) in coordinates.iter() {
            if self.notes[*row][*col] == note && (*row, *col) != (x, y) {
                any_progress |= self.clear_notes_obvious_set_based(coordinates, note);
            }
        }
        any_progress
    }

    fn clear_notes_obvious_set_based(&mut self, coordinates: &Coordinates, pair_note: u16) -> bool {
        let mut any_progress = false;
        for (row, col) in coordinates.iter() {
            let note = &mut self.notes[*row][*col];
            if (*note & pair_note) != *note && (*note & pair_note) != 0 {
                any_progress = true;
                *note &= !pair_note;
            }
        }
        any_progress
    }

    pub fn get_hidden(&mut self, coordinates: &Coordinates, value: usize) -> Option<Point> {
        let mut count_values = 0;
        let mut row_found = 0;
        let mut col_found = 0;
        for (row, col) in coordinates.iter() {
            if Self::is_set(&self.notes[*row][*col], value) {
                count_values += 1;
                row_found = *row;
                col_found = *col;
            }
        }
        if count_values == 1 {
            Some((row_found, col_found))
        } else {
            None
        }
    }

    pub fn use_square_methods(&mut self) -> bool {
        crate::solve::pointing_sets::Handler::new(&mut self.notes).handle() ||
            crate::solve::hidden_sets::use_hidden_sets(&mut self.notes)
    }
}

#[cfg(test)]
mod tests {
    include!("ut/test_notes_manager.rs");
}
