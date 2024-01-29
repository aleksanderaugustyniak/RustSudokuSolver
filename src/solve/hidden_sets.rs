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
    for (first, map) in values_map.iter().enumerate() {
        if map.count_ones() == 2 {
            //first is candidate
            for second in first + 1..GRID_SIZE {
                if values_map[second] == values_map[first] {
                    // candidate found -> check candidate
                    let candidate = (1 << second) | (1 << first);
                    let (row_first, col_first) = cells[first];
                    let (row_second, col_second) = cells[second];

                    notes[row_first][col_first] = candidate;
                    notes[row_second][col_second] = candidate;
                    result = true;
                }
            }
        }
    }
    result
}

fn check_candidate(first: u16, second: u16, candidate: u16) -> bool {
    (candidate & first) == candidate && (candidate & second) == candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_candidate() {
        let actual = check_candidate(0b000_000_011, 0b000_000_110, 0b000_000_011);
        assert!(!actual);
        let actual = check_candidate(0b100_010_011, 0b010_101_011, 0b000_000_011);
        assert!(actual);
    }
}
