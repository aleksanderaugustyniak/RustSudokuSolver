use fltk::{app, menu, prelude::*, enums::*, window};
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::play_board::PlayBoard;
use crate::gui::control_panel::ControlPanel;

const MENU_WIDTH: i32 = 25;
const BUTTON_SIZE: i32 = 50;
const GRID_SIZE: usize = 9;
const BOARD_OFFSET_TOP: i32 = 2*MENU_WIDTH;
const WINDOW_WIDTH: i32 = MENU_WIDTH + BOARD_OFFSET_TOP*3 + BUTTON_SIZE*GRID_SIZE as i32;

pub struct Game {
    app: app::App,
    window: window::Window,
    menu_bar: menu::MenuBar,
    play_board: Rc<RefCell<PlayBoard>>,
    control_panel: Rc<RefCell<ControlPanel>>,
}

impl Game {
    pub fn new() -> Self {
        let play_board = Rc::new(RefCell::new(PlayBoard::new()));
        let control_panel = Rc::new(RefCell::new(ControlPanel::new(Rc::clone(&play_board))));
        
        Game {
            app: app::App::default(),
            window: window::Window::new(100, 80, WINDOW_WIDTH, WINDOW_WIDTH, "Sudoku"),
            menu_bar: menu::MenuBar::new(0, 0, WINDOW_WIDTH, MENU_WIDTH, ""),
            play_board,
            control_panel,
        }
    }

    pub fn play(&mut self) {
        self.window.set_color(Color::White);

        self.display_menu();
        self.play_board.borrow_mut().display(); 
        self.control_panel.borrow_mut().display();

        self.window.make_resizable(true);
        self.window.end();
        self.window.show();
        self.app.run().unwrap();
    }

    fn display_menu(&mut self) {
        let mut file_menu = menu::MenuButton::new(0, 0, 60, MENU_WIDTH, "Board");
        file_menu.add_choice("Save");
        file_menu.add_choice("Read");
        file_menu.add_choice("Clear");
        let file_menu_clone = file_menu.clone();
        let play_board_clone = Rc::clone(&self.play_board);

        file_menu.set_callback(move |_| {
            match file_menu_clone.value() {
                0 => {
                    if let Err(err) = play_board_clone.borrow().to_json() {
                        eprintln!("Error writing to JSON file: {}", err);
                    }
                }
                1 => {
                    if let Err(err) = play_board_clone.borrow_mut().from_json() {
                        eprintln!("Error updating from JSON file: {}", err);
                    }
                }
                2 => {
                    (*play_board_clone.borrow_mut()).clear();
                }
                _ => {}
            }
        });
    }
}
