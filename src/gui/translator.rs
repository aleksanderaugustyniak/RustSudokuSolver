use fltk::prelude::WidgetExt;
use crate::gui::board::*;

pub fn labels(board: &Board) -> Labels {
    let mut output: Labels = Default::default();
    for (row, buttons) in board.iter().enumerate() {
        for (col, button) in buttons.iter().enumerate() {
            output[row][col] = button.label();
        }
    }
    output
}
