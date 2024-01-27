use crate::common::grid_size::GRID_SIZE;
use crate::solve::notes::Notes;
use crate::solve::coordinates::*;
type Counters = [u8; GRID_SIZE];

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
    let counters = crate::solve::count_notes::count(notes, cells);
    for (index, count) in counters.iter().enumerate() {
        // TODO: consider placing here setting hidden ones from solver

    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
}
