use crate::common::grid_size::GRID_SIZE;

pub type Point = (usize, usize);
pub type Coordinates = [Point; GRID_SIZE];

pub fn get_row_coordinates(row: usize) -> Coordinates {
    let mut c: Coordinates = Default::default();
    for (i, coord) in c.iter_mut().enumerate() {
        *coord = (row, i);
    }
    c
}

pub fn get_col_coordinates(col: usize) -> Coordinates {
    let mut c: Coordinates = Default::default();
    for (i, coord) in c.iter_mut().enumerate() {
        *coord = (i, col);
    }
    c
}

pub fn get_square_coordinates((square_x, square_y): Point) -> Coordinates {
    let mut c: Coordinates = Default::default();
    let mut index = 0;
    for row in [3 * square_x, 3 * square_x + 1, 3 * square_x + 2] {
        for col in [3 * square_y, 3 * square_y + 1, 3 * square_y + 2] {
            c[index] = (row, col);
            index += 1;
        }
    }
    c
}

pub fn perform_for_all_sets<F>(mut action: F) -> bool where F: FnMut(&Coordinates) -> bool {
    let mut result = false;
    for index in 0..GRID_SIZE {
        let all_coordinates = [
            get_row_coordinates(index),
            get_col_coordinates(index),
            get_square_coordinates((index % 3, index / 3)),
        ];
        for coordinates in all_coordinates {
            result |= action(&coordinates);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row_coordinates() {
        let row = 2;
        let coordinates = get_row_coordinates(row);
        for (i, (x, y)) in coordinates.iter().enumerate() {
            assert_eq!(*x, row);
            assert_eq!(*y, i);
        }
    }

    #[test]
    fn test_get_col_coordinates() {
        let col = 3;
        let coordinates = get_col_coordinates(col);
        for (i, (x, y)) in coordinates.iter().enumerate() {
            assert_eq!(*x, i);
            assert_eq!(*y, col);
        }
    }

    #[test]
    fn test_get_square_coordinates() {
        let square_coordinates = (0, 2);
        let expected: Coordinates = [
            (0, 6),
            (0, 7),
            (0, 8),
            (1, 6),
            (1, 7),
            (1, 8),
            (2, 6),
            (2, 7),
            (2, 8),
        ];
        let actual = get_square_coordinates(square_coordinates);
        assert_eq!(actual, expected);
    }
}
