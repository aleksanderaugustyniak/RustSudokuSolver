use crate::common::grid_size::GRID_SIZE;
use crate::solve::notes::Notes;
use crate::solve::coordinates::*;

pub fn use_hidden_sets(notes: &mut Notes) -> bool {
    let mut result = false;
    for i in 0..GRID_SIZE {
        result |= check_hidden_set(notes, &get_row_coordinates(i));
        result |= check_hidden_set(notes, &get_col_coordinates(i));
        result |= check_hidden_set(notes, &get_square_coordinates((i / 3, i % 3)));
    }
    result
}

fn check_hidden_set(notes: &mut Notes, cells: &Coordinates) -> bool {
    let mut result = false;
    let values_map = crate::solve::map_notes::map(notes, cells);
    for first in 0..GRID_SIZE - 1 {
        if values_map[first].count_ones() == 2 {
            for second in first + 1..GRID_SIZE {
                if values_map[second] == values_map[first] {
                    result |= clear_hidden_set(notes, cells, candidate(first, second));
                }
            }
        }
        // TODO: check hidden triples
    }
    result
}

fn clear_hidden_set(notes: &mut Notes, cells: &Coordinates, candidate: u16) -> bool {
    let mut result = false;
    for (row, col) in cells.iter() {
        if (notes[*row][*col] & candidate) != 0 && (notes[*row][*col] | candidate) != candidate {
            notes[*row][*col] &= candidate;
            result |= true;
        }
    }
    result
}

fn candidate(first: usize, second: usize) -> u16 {
    (1 << second) | (1 << first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_candidate() {
        assert_eq!(candidate(0, 1), 0b11);
        assert_eq!(candidate(8, 6), 0b101_000_000);
    }
}
