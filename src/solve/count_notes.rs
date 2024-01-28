use crate::common::grid_size::GRID_SIZE;
use crate::solve::notes::Notes;
use crate::solve::coordinates::*;

pub type Counters = [u16; GRID_SIZE]; //TODO: it's same as HelperNotes -> extract common type

pub fn count(notes: &Notes, cells: &Coordinates) -> Counters {
    let mut counters: Counters = Default::default();
    for (index, (row, col)) in cells.iter().enumerate() {
        for value in 1..=GRID_SIZE {
            if is_set(notes[*row][*col], value) {
                // counters[value - 1] += 1;
                set_note(&mut counters[value - 1], index as u8);
            }
        }
    }
    counters
}

// TODO: remove copied code -> extract Bitset class
fn is_set(note: u16, position: usize) -> bool {
    let mask = 1 << (position - 1);
    (note & mask) != 0
}

fn set_note(note: &mut u16, bit: u8) {
    *note |= 1 << bit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_set_basic() {
        for i in 1..=GRID_SIZE {
            assert!(!is_set(0, i));
            assert!(is_set(0b111_111_111, i));
        }
    }

    #[test]
    fn test_is_set() {
        let bitset = 0b111_000_111;
        assert!(is_set(bitset, 1));
        assert!(is_set(bitset, 8));
        assert!(!is_set(bitset, 4));
    }

    #[test]
    fn test_count() {
        let notes: Notes = [
            [
                0, 0, 0b000_001_011, 0b000_100_010, 0, 0b010_101_010, 0b110_001_001, 0b100_001_011,
                0b010_001_010,
            ],
            [0, 0b001_001_010, 0b001_001_010, 0, 0, 0, 0b011_001_100, 0b000_001_110, 0b011_001_010],
            [
                0b000_000_011, 0, 0, 0b000_000_110, 0b000_001_100, 0b000_001_010, 0b001_011_101, 0,
                0b001_001_010,
            ],
            [0, 0, 0b100_010_011, 0b101_010_000, 0, 0b001_001_001, 0b101_011_000, 0b100_011_010, 0],
            [0, 0b000_010_010, 0b100_110_010, 0, 0b000_010_000, 0, 0b101_010_000, 0b100_010_010, 0],
            [0, 0b000_010_001, 0b100_010_101, 0b100_010_000, 0, 0b000_001_001, 0b110_011_000, 0, 0],
            [0, 0, 0b101_011_101, 0b001_010_100, 0b000_010_100, 0b001_000_000, 0, 0, 0b000_001_000],
            [0b000_000_110, 0b011_000_010, 0b001_000_110, 0, 0, 0, 0b000_100_100, 0b000_000_100, 0],
            [0, 0, 0b000_011_111, 0b000_110_110, 0, 0b000_100_010, 0, 0, 0],
        ];

        let coordinates = get_row_coordinates(2);
        let actual = count(&notes, &coordinates);
        let expected = [
            0b001_000_001, 0b100_101_001, 0b001_011_000, 0b101_110_000, 0b001_000_000, 0,
            0b101_000_000, 0, 0,
        ];
        assert_eq!(actual, expected);
    }
}
