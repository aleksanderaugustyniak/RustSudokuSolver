use fltk::{ prelude::* };
use serde::Serialize;
use serde::Deserialize;
use crate::gui::board::*;

type Labels = [[String; GRID_SIZE]; GRID_SIZE];

#[derive(Serialize, Deserialize)]
pub struct Saver {}

impl Saver {
    pub fn to_json(file_path: &str, board: &Board) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = serde_json::to_string_pretty(&Self::prepare_json_content(&board))?;
        std::fs::write(file_path, json_content)?;

        Ok(())
    }

    fn prepare_json_content(board: &Board) -> Labels {
        let mut output: [[String; GRID_SIZE]; GRID_SIZE] = Default::default();
        for (row, buttons) in board.iter().enumerate() {
            for (col, button) in buttons.iter().enumerate() {
                output[row][col] = button.label();
            }
        }
        output
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
}
