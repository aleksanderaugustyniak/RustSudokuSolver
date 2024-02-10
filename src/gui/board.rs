use fltk::button::Button;
use crate::common::grid_size::GRID_SIZE;

pub type Board = [[Button; GRID_SIZE]; GRID_SIZE];
pub type Labels = [[String; GRID_SIZE]; GRID_SIZE];
