use serde::Serialize;
use serde::Deserialize;
use crate::gui::board::*;
use crate::gui::translator::*;

#[derive(Serialize, Deserialize)]
pub struct Saver;

pub fn to_json(file_path: &str, board: &Board) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(&labels(board))?;
    Ok(std::fs::write(file_path, json_content)?)
}

pub fn from_json(file_path: &str) -> Result<Labels, Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&json_content)?)
}
