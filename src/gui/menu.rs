use fltk::{ menu, prelude::* };
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::play_board::PlayBoard;
use crate::gui::consts::MENU_WIDTH;

const SAVE: &str = "Save";
const READ: &str = "Read";
const CLEAR: &str = "Clear";
const SOLVE: &str = "Solve";
const SHOW_NOTES: &str = "Show notes";

const MENU_LENGTH: i32 = 60;

pub struct Menu {
    _menu_bar: menu::MenuBar,
    file_menu: menu::MenuButton,
    solve_menu: menu::MenuButton,
    board: Rc<RefCell<PlayBoard>>,
}

impl Menu {
    pub fn new(play_board: Rc<RefCell<PlayBoard>>, window_width: i32) -> Self {
        Menu {
            _menu_bar: menu::MenuBar::new(0, 0, window_width, MENU_WIDTH, ""),
            file_menu: menu::MenuButton::new(0, 0, MENU_LENGTH, MENU_WIDTH, "Board"),
            solve_menu: menu::MenuButton::new(MENU_LENGTH, 0, MENU_LENGTH, MENU_WIDTH, SOLVE),
            board: Rc::clone(&play_board),
        }
    }

    pub fn display(&mut self) {
        self.file_menu.add_choice(SAVE);
        self.file_menu.add_choice(READ);
        self.file_menu.add_choice(CLEAR);
        self.set_file_callback();
        self.solve_menu.add_choice(SOLVE);
        self.solve_menu.add_choice(SHOW_NOTES);
        self.set_solve_callback();
    }

    fn set_file_callback(&mut self) {
        let file_menu_clone = self.file_menu.clone();
        let board_clone = Rc::clone(&self.board);

        self.file_menu.set_callback(move |_| {
            if let Some(choice) = file_menu_clone.choice() {
                match choice.as_str() {
                    SAVE => {
                        if let Err(err) = board_clone.borrow().to_json() {
                            eprintln!("Error writing to JSON file: {}", err);
                        }
                    }
                    READ => {
                        if let Err(err) = board_clone.borrow_mut().read_from_file() {
                            eprintln!("Error updating from JSON file: {}", err);
                        }
                    }
                    CLEAR => {
                        (*board_clone.borrow_mut()).clear();
                    }
                    _ => {}
                }
            }
        });
    }

    fn set_solve_callback(&mut self) {
        let solve_menu_clone = self.solve_menu.clone();
        let board_clone = Rc::clone(&self.board);

        self.solve_menu.set_callback(move |_| {
            if let Some(choice) = solve_menu_clone.choice() {
                match choice.as_str() {
                    SOLVE => {
                        board_clone.borrow_mut().solve_puzzle();
                    }
                    SHOW_NOTES => {
                        board_clone.borrow_mut().show_notes();
                    }
                    _ => {}
                }
            }
        });
    }
}
