use crate::common::grid_size::GRID_SIZE;
use crate::solve::notes::Notes;

pub struct Handler<'a> {
    notes: &'a mut Notes,
    progress: bool,
}

impl<'a> Handler<'a> {
    pub fn new(notes: &'a mut Notes) -> Self {
        Handler {
            notes,
            progress: false,
        }
    }

    pub fn handle(&mut self) -> bool {
        for start_x in [0, 3, 6] {
            for start_y in [0, 3, 6] {
                self.handle_pointing_on_square_row(start_x, start_y);
                self.handle_pointing_on_square_col(start_x, start_y);
            }
        }
        self.progress
    }

    fn handle_pointing_on_square_row(&mut self, row: usize, col: usize) {
        let notes = self.get_square_row_notes(row, col);
        for (i, note) in Self::get_pointing_from_notes(notes).iter().enumerate() {
            if *note != 0 {
                self.use_pointing_set_on_row(*note, row + i, col);
            }
        }
    }

    fn handle_pointing_on_square_col(&mut self, row: usize, col: usize) {
        let notes = self.get_square_col_notes(row, col);
        for (i, note) in Self::get_pointing_from_notes(notes).iter().enumerate() {
            if *note != 0 {
                self.use_pointing_set_on_col(*note, row, col + i);
            }
        }
    }

    fn use_pointing_set_on_row(&mut self, note: u16, row: usize, col: usize) {
        for i in 0..GRID_SIZE {
            if i / 3 != col / 3 {
                let cell = &mut self.notes[row][i];
                if (*cell & note) != 0 {
                    self.progress = true;
                }
                *cell &= !note;
            }
        }
    }

    fn use_pointing_set_on_col(&mut self, note: u16, row: usize, col: usize) {
        for i in 0..GRID_SIZE {
            if i / 3 != row / 3 {
                let cell = &mut self.notes[i][col];
                if (*cell & note) != 0 {
                    self.progress = true;
                }
                *cell &= !note;
            }
        }
    }

    fn get_square_row_notes(&self, row: usize, col: usize) -> [u16; 3] {
        let mut notes: [u16; 3] = Default::default();
        for (i, note) in notes.iter_mut().enumerate() {
            *note = self.get_notes_on_single_square_row(row + i, col);
        }
        notes
    }

    fn get_square_col_notes(&self, row: usize, col: usize) -> [u16; 3] {
        let mut notes: [u16; 3] = Default::default();
        for (i, note) in notes.iter_mut().enumerate() {
            *note = self.get_notes_on_single_square_col(col + i, row);
        }
        notes
    }

    fn get_notes_on_single_square_row(&self, row: usize, start_col: usize) -> u16 {
        let mut row_notes = 0;
        for col in Self::square_iter(start_col) {
            row_notes |= self.notes[row][col];
        }
        row_notes
    }

    fn get_notes_on_single_square_col(&self, col: usize, start_row: usize) -> u16 {
        let mut row_notes = 0;
        for row in Self::square_iter(start_row) {
            row_notes |= self.notes[row][col];
        }
        row_notes
    }

    fn get_pointing_from_notes(notes: [u16; 3]) -> [u16; 3] {
        let mut pointing: [u16; 3] = Default::default();
        pointing[0] = notes[0] & !(notes[1] | notes[2]);
        pointing[1] = notes[1] & !(notes[0] | notes[2]);
        pointing[2] = notes[2] & !(notes[1] | notes[0]);
        pointing
    }

    fn square_iter(position: usize) -> [usize; 3] {
        let start = position - (position % 3);
        [start, start + 1, start + 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squre_iter() {
        assert_eq!(Handler::square_iter(2), [0, 1, 2]);
        assert_eq!(Handler::square_iter(3), [3, 4, 5]);
    }

    #[test]
    fn test_pointing() {
        assert_eq!(Handler::get_pointing_from_notes([0, 0, 0]), [0, 0, 0]);
        assert_eq!(
            Handler::get_pointing_from_notes([0b111_111_111, 0b111_111_111, 0b111_111_111]),
            [0, 0, 0]
        );
        assert_eq!(
            Handler::get_pointing_from_notes([0b110_110_011, 0b001_110_010, 0b001_001_001]),
            [0b_110_000_000, 0, 0b_000_001_000]
        );
    }

    const NOTES1: Notes = [
        [
            0, 0, 0b000_001_011, 0b000_100_010, 0, 0b010_101_010, 0b110_001_001, 0b100_001_011,
            0b010_001_010,
        ],
        [0, 0b001_001_010, 0b001_001_010, 0, 0, 0, 0b011_001_100, 0b000_001_110, 0b011_001_010],
        [
            0b000_000_011, 0, 0, 0b000_000_110, 0b000_001_100, 0b000_001_010, 0b001_011_101, 0,
            0b001_001_010,
        ],
        [
            0, 0b000_010_011, 0b100_010_011, 0b101_010_000, 0, 0b001_001_001, 0b101_011_000, 0b100_011_010,
            0,
        ],
        [0, 0b000_010_010, 0b100_110_010, 0, 0b000_010_000, 0, 0b101_010_000, 0b100_010_010, 0],
        [
            0, 0b000_010_001, 0b100_010_101, 0b100_010_000, 0, 0b000_001_001, 0b110_011_000, 0b100_011_000,
            0,
        ],
        [
            0b100_000_101, 0, 0b101_011_101, 0b001_010_100, 0b000_010_100, 0b001_000_000, 0, 0,
            0b000_001_000,
        ],
        [0b000_000_110, 0b011_000_010, 0b001_000_110, 0, 0, 0, 0b000_100_100, 0b000_000_100, 0],
        [
            0b000_000_111, 0b000_011_011, 0b000_011_111, 0b000_110_110, 0, 0b000_100_010,
            0b000_101_101, 0, 0,
        ],
    ];

    fn check_notes(notes_index: usize, cross_index: usize, actual: u16, expected: u16, name: &str) {
        assert_eq!(
            actual,
            expected,
            "Wrong notes for {} {}, square starting on {}, actual {:b}, expected {:b}",
            name,
            notes_index,
            cross_index,
            actual,
            expected
        );
    }

    fn check_row_notes(sut: &mut Handler, row: usize, col: usize, expected: u16) {
        let actual = sut.get_notes_on_single_square_row(row, col);
        check_notes(row, col, actual, expected, "row");
    }

    fn check_col_notes(sut: &mut Handler, row: usize, col: usize, expected: u16) {
        let actual = sut.get_notes_on_single_square_col(col, row);
        check_notes(col, row, actual, expected, "col");
    }

    #[test]
    fn test_get_notes() {
        let mut binding = NOTES1;
        let mut sut = Handler::new(&mut binding);
        check_row_notes(&mut sut, 0, 0, 0b000_001_011);
        check_row_notes(&mut sut, 1, 0, 0b001_001_010);
        check_row_notes(&mut sut, 2, 0, 0b000_000_011);
        assert_eq!(sut.get_square_row_notes(0, 0), [0b000_001_011, 0b001_001_010, 0b000_000_011]);

        check_col_notes(&mut sut, 0, 0, 0b000_000_011);
        check_col_notes(&mut sut, 0, 1, 0b001_001_010);
        check_col_notes(&mut sut, 0, 2, 0b001_001_011);
        assert_eq!(sut.get_square_col_notes(0, 0), [0b000_000_011, 0b001_001_010, 0b001_001_011]);

        check_row_notes(&mut sut, 2, 3, 0b000_001_110);
        check_col_notes(&mut sut, 0, 6, 0b111_011_101);
        check_col_notes(&mut sut, 3, 8, 0);
    }
}
