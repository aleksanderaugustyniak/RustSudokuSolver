use fltk::prelude::WidgetExt;
use crate::gui::board::Board;
use crate::common::puzzle::Puzzle;

pub fn read_puzzle(board: &Board) -> Puzzle {
    let mut output: Puzzle = Default::default();
    for (row, buttons) in board.iter().enumerate() {
        for (col, button) in buttons.iter().enumerate() {
            output[row][col] = match button.label().parse::<u8>() {
                Ok(parsed_number) => { parsed_number }
                Err(_) => { 0 }
            };
        }
    }
    output
}
