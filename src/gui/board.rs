use fltk::button::Button;

pub const GRID_SIZE: usize = 9;

pub type Board = [[Button; GRID_SIZE]; GRID_SIZE];
