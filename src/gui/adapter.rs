use fltk::prelude::WidgetExt;
use crate::gui::board::Board;
use crate::common::puzzle::Puzzle;

pub fn read_puzzle(board: &Board) -> Puzzle {
    let mut output: Puzzle = Default::default();
    for (row, buttons) in board.iter().enumerate() {
        for (col, button) in buttons.iter().enumerate() {
            match button.label().parse::<u8>() {
                Ok(parsed_number) => {
                    output[row][col] = parsed_number;
                }
                Err(_) => {
                    output[row][col] = 0;
                }
            }
        }
    }
    output
}
