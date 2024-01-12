use fltk::{app, menu, prelude::*, button::*, enums::*, window};
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::play_board::PlayBoard;

const MENU_WIDTH: i32 = 25;
const BUTTON_SIZE: i32 = 50;
const GRID_SIZE: usize = 9;
const BOARD_OFFSET_LEFT: i32 = 30;
const BOARD_OFFSET_TOP: i32 = 2*MENU_WIDTH;
const WINDOW_WIDTH: i32 = MENU_WIDTH + BOARD_OFFSET_TOP*3 + BUTTON_SIZE*GRID_SIZE as i32;
const LIGHT_BUTTON_COLOR: Color = Color::from_rgb(200, 200, 200);
const HIGHLIGHTED_BUTTON_COLOR: Color = Color::from_rgb(100, 100, 250);

pub struct Game {
    app: app::App,
    window: window::Window,
    menu_bar: menu::MenuBar,
    control_panel: Rc<RefCell<[Button; GRID_SIZE + 1]>>,
    play_board: Rc<RefCell<PlayBoard>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            app: app::App::default(),
            window: window::Window::new(100, 80, WINDOW_WIDTH, WINDOW_WIDTH, "Sudoku"),
            menu_bar: menu::MenuBar::new(0, 0, WINDOW_WIDTH, MENU_WIDTH, ""),
            control_panel: Default::default(),
            play_board: Rc::new(RefCell::new(PlayBoard::new())),
        }
    }

    pub fn play(&mut self) {
        self.window.set_color(Color::White);

        self.display_menu();
        self.play_board.borrow_mut().display(); 
        self.display_control_panel();

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

    fn display_control_panel(&mut self) {
        for (number, button) in self.control_panel.borrow_mut().iter_mut().enumerate() {
            *button = Button::new(
                BOARD_OFFSET_LEFT + (number) as i32 * BUTTON_SIZE,
                BUTTON_SIZE * GRID_SIZE as i32 + BOARD_OFFSET_TOP + 25,
                BUTTON_SIZE,
                BUTTON_SIZE,
                "",
            );
            button.set_label(&format!("{}", number + 1));
            button.set_color(LIGHT_BUTTON_COLOR);
        }
        self.control_panel.borrow_mut()[GRID_SIZE].set_label("");
        self.set_control_panel_callbacks();
    }

    fn set_control_panel_callbacks(&mut self) {
        for control_button in self.control_panel.borrow_mut().iter_mut() {
            let control_panel_clone = Rc::clone(&self.control_panel);
            let play_board_clone = Rc::clone(&self.play_board);

            control_button.set_callback(move |button: &mut Button| {
                play_board_clone.borrow_mut().set_number(&button.label().to_string());
                for but in (*control_panel_clone.borrow_mut()).iter_mut() {
                    but.set_color(LIGHT_BUTTON_COLOR);
                    but.redraw();
                }
                if button.label() != "" {play_board_clone.borrow_mut().highlight(&button.label())}
                    else{play_board_clone.borrow_mut().clear_color();};
                button.set_color(HIGHLIGHTED_BUTTON_COLOR);
            });
        }
    }
}
