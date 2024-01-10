use fltk::{prelude::*, button::*};
use serde::Serialize;
use serde::Deserialize;

const GRID_SIZE: usize = 9;

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

    fn prepare_json_content(board: &[[Button; GRID_SIZE]; GRID_SIZE]) -> [[String; GRID_SIZE]; GRID_SIZE] {
        let mut output : [[String; GRID_SIZE]; GRID_SIZE] = Default::default();
        for (row, buttons) in board.iter().enumerate() {
            for (col, button) in buttons.iter().enumerate() {
                output[row][col] = button.label();
            }
        }
        output
    }
}
