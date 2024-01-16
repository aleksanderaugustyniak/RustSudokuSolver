use fltk::{ prelude::* };
use serde::Serialize;
use serde::Deserialize;
use crate::gui::board::*;
use crate::gui::translator::*;

#[derive(Serialize, Deserialize)]
pub struct Saver;

pub fn to_json(file_path: &str, board: &Board) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(&labels(board))?;
    std::fs::write(file_path, json_content)?;

    Ok(())
}

pub fn from_json(file_path: &str, board: &mut Board) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(file_path)?;
    let labels: Labels = serde_json::from_str(&json_content)?;
    for (row, buttons) in board.iter_mut().enumerate() {
        for (col, button) in buttons.iter_mut().enumerate() {
            button.set_label(&labels[row][col]);
            button.redraw();
        }
    }

    Ok(())
}
