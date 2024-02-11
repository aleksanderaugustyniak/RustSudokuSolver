use crate::common::grid_size::GRID_SIZE;
use crate::solve::notes::Notes;
use crate::solve::coordinates::*;

pub fn use_hidden_sets(notes: &mut Notes) -> bool {
    perform_for_all_sets(|coordinates| { check_hidden_set(notes, coordinates) })
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
        let note = notes[*row][*col];
        let is_not_set = (note & candidate) != 0;
        if is_not_set {
            let is_not_candidate = (note | candidate) != candidate;
            notes[*row][*col] &= candidate;
            result |= is_not_candidate;
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
