use fltk::{prelude::*, button::*};
use serde::Serialize;
use serde::Deserialize;

const GRID_SIZE: usize = 9;
type Labels = [[String; GRID_SIZE]; GRID_SIZE];

#[derive(Serialize, Deserialize)]
pub struct Saver {

}

impl Saver {
    pub fn new() -> Self {
        Saver {

        }
    }

    pub fn to_json(file_path: &str, board: &[[Button; GRID_SIZE]; GRID_SIZE]) -> Result<(), Box<dyn std::error::Error>>
    {
        let json_content = serde_json::to_string_pretty(&Self::prepare_json_content(&board))?;
        std::fs::write(file_path, json_content)?;

        Ok(())
    }

    fn prepare_json_content(board: &[[Button; GRID_SIZE]; GRID_SIZE]) -> Labels {
        let mut output : [[String; GRID_SIZE]; GRID_SIZE] = Default::default();
        for (row, buttons) in board.iter().enumerate() {
            for (col, button) in buttons.iter().enumerate() {
                output[row][col] = button.label();
            }
        }
        output
    }

    pub fn from_json(file_path: &str, board: &mut [[Button; GRID_SIZE]; GRID_SIZE]) -> Result<(), Box<dyn std::error::Error>>
    {
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
