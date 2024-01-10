use fltk::{app, menu, prelude::*, button::*, enums::*, group::*, window};
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::save_handler::Saver;

const MENU_WIDTH: i32 = 25;
const BUTTON_SIZE: i32 = 50;
const GRID_SIZE: usize = 9;
const BOARD_OFFSET_LEFT: i32 = 30;
const BOARD_OFFSET_TOP: i32 = 2*MENU_WIDTH;
const WINDOW_WIDTH: i32 = MENU_WIDTH + BOARD_OFFSET_TOP*3 + BUTTON_SIZE*GRID_SIZE as i32;
const LIGHT_BUTTON_COLOR: Color = Color::from_rgb(200, 200, 200);
const DARK_BUTTON_COLOR: Color = Color::from_rgb(150, 150, 150);
const HIGHLIGHTED_BUTTON_COLOR: Color = Color::from_rgb(100, 100, 250);
type Board = [[Button; GRID_SIZE]; GRID_SIZE];

pub struct Game {
    app: app::App,
    window: window::Window,
    menu_bar: menu::MenuBar,
    current_number: Rc<RefCell<String>>,
    control_panel: Rc<RefCell<[Button; GRID_SIZE]>>,
    play_grid: Rc<RefCell<Board>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            app: app::App::default(),
            window: window::Window::new(100, 80, WINDOW_WIDTH, WINDOW_WIDTH, "Sudoku"),
            menu_bar: menu::MenuBar::new(0, 0, WINDOW_WIDTH, MENU_WIDTH, ""),
            current_number: Rc::new(RefCell::new("1".to_string())),
            control_panel: Default::default(),
            play_grid: Default::default(),
        }
    }

    pub fn play(&mut self) {
        self.window.set_color(Color::White);

        self.display_menu();
        self.display_play_grid(); 
        self.display_control_panel();

        self.window.make_resizable(true);
        self.window.end();
        self.window.show();
        self.app.run().unwrap();
    }

    fn display_menu(&mut self) {
        let mut file_menu = menu::MenuButton::new(0, 0, 60, MENU_WIDTH, "File");
        file_menu.add_choice("Save");
        let play_grid_clone = Rc::clone(&self.play_grid);
        file_menu.set_callback(move |_| {
            if let Err(err) = Saver::to_json("boards/board.json", &play_grid_clone.borrow()) {
                eprintln!("Error writing to JSON file: {}", err);
            }

        });
    }

    fn display_play_grid(&mut self) {
        let mut grid = Pack::new(
            10,
            10,
            BUTTON_SIZE * GRID_SIZE as i32,
            BUTTON_SIZE * GRID_SIZE as i32,
            "");
        grid.make_resizable(true);
            
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                grid.end();
                self.display_button(row, col);
            }
        }
        grid.end();
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
        self.set_control_panel_callbacks();
    }

    fn set_control_panel_callbacks(&mut self) {
        for control_button in self.control_panel.borrow_mut().iter_mut() {
            let current_number_clone = Rc::clone(&self.current_number);
            let control_panel_clone = Rc::clone(&self.control_panel);
            let play_grid_clone = Rc::clone(&self.play_grid);

            control_button.set_callback(move |button: &mut Button| {
                *current_number_clone.borrow_mut() = button.label().to_string();
                for but in (*control_panel_clone.borrow_mut()).iter_mut() {
                    but.set_color(LIGHT_BUTTON_COLOR);
                    but.redraw();
                }
                Self::highlight_play_buttons(&mut (*play_grid_clone.borrow_mut()), &button.label());
                button.set_color(HIGHLIGHTED_BUTTON_COLOR);
            });
        }
    }

    fn highlight_play_buttons(board: &mut Board, label: &str) {
        for (row, play_row) in board.iter_mut().enumerate() {
            for (col, button) in play_row.iter_mut().enumerate() {
                let color = if button.label() == label {HIGHLIGHTED_BUTTON_COLOR} else {Self::get_square_color(row, col)};
                button.set_color(color);
                button.redraw();
            }
        }
    }

    fn display_button(&self, row: usize, col: usize) {
        self.play_grid.borrow_mut()[row][col] = Button::new(
            BOARD_OFFSET_LEFT + col as i32 * BUTTON_SIZE,
            BOARD_OFFSET_TOP + row as i32 * BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_SIZE,
            "",
        );

        self.set_play_button_callback(row, col);
        self.play_grid.borrow_mut()[row][col].set_color(Self::get_square_color(row, col));
    }

    fn get_square_color( row: usize, col: usize) -> Color {
        let square_id = (row / 3) + (col / 3);
        if square_id % 2 == 1 {DARK_BUTTON_COLOR} else {LIGHT_BUTTON_COLOR}
    }

    fn set_play_button_callback(&self, row: usize, col: usize) {
        let button_label = Rc::clone(&self.current_number);
        self.play_grid.borrow_mut()[row][col].set_callback(move |button: &mut Button| {
            button.set_label(&format!("{}", button_label.borrow()));
            button.set_color(HIGHLIGHTED_BUTTON_COLOR);
        });
    }
}
