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
            for coordinates in all_coordinates(index) {
                self.set_notes(&coordinates);
            }
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

    fn get_notes(&self, coordinates: &Coordinates) -> u16 {
        let mut note: u16 = FILLED_BITSET;
        for (row, col) in coordinates.iter() {
            let cell = self.puzzle[*row][*col];
            if cell != 0 {
                Self::unset_note(&mut note, cell);
            }
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
        if Self::is_valid_position(bit as usize) {
            *note &= !(1 << (bit - 1));
        }
    }

    fn is_set(note: u16, position: usize) -> bool {
        if Self::is_valid_position(position) {
            let mask = 1 << (position - 1);
            (note & mask) != 0
        } else {
            false
        }
    }

    fn is_valid_position(position: usize) -> bool {
        position > 0 && position <= GRID_SIZE
    }

    pub fn set_obvious_pairs(&mut self) -> bool {
        let mut any_progress = false;
        for index in 0..GRID_SIZE {
            for coordinates in all_coordinates(index) {
                any_progress |= self.perform_obvious_set(&coordinates);
            }
        }
        any_progress
    }

    fn perform_obvious_set(&mut self, coordinates: &Coordinates) -> bool {
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
            let note = self.notes[*row][*col];
            if note.count_ones() <= 3 && note != 0 {
                result |= self.handle_triple(coordinates, index, note);
            }
        }
        result
    }

    fn handle_triple(&mut self, coordinates: &Coordinates, index: usize, first_note: u16) -> bool {
        if index >= GRID_SIZE - 2 {
            return false; // can't find triple on last or previous one
        }
        let mut result = false;
        for second in index + 1..GRID_SIZE - 1 {
            let (row, col) = coordinates[second];
            let second_note = self.notes[row][col] | first_note;
            if second_note.count_ones() <= 3 && self.notes[row][col] != 0 {
                for third in second + 1..GRID_SIZE {
                    let (row3, col3) = coordinates[third];
                    let note = second_note | self.notes[row3][col3];
                    if note.count_ones() <= 3 && self.notes[row3][col3] != 0 {
                        result |= self.clear_notes_obvious_set_based(coordinates, note);
                    }
                }
            }
        }
        result
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
            let masked_note = *note & pair_note;
            if masked_note != *note && masked_note != 0 {
                any_progress = true;
                *note &= !pair_note;
            }
        }
        any_progress
    }

    pub fn get_hidden(&self, coordinates: &Coordinates, value: usize) -> Option<Point> {
        let mut found = false;
        let mut coordinate_found = None;
        for (row, col) in coordinates.iter() {
            if Self::is_set(self.notes[*row][*col], value) {
                if found {
                    return None;
                }
                found = true;
                coordinate_found = Some((*row, *col));
            }
        }
        coordinate_found
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
