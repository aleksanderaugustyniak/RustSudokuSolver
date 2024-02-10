use fltk::{ app, prelude::*, window };
use fltk_theme::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::menu::Menu;
use crate::gui::play_board::PlayBoard;
use crate::gui::control_panel::ControlPanel;

const WINDOW_WIDTH: i32 = 650;

pub struct Game {
    app: app::App,
    window: window::Window,
    menu: Menu,
    play_board: Rc<RefCell<PlayBoard>>,
    control_panel: Rc<RefCell<ControlPanel>>,
}

impl Game {
    pub fn new() -> Self {
        let window = window::Window::new(100, 80, WINDOW_WIDTH, WINDOW_WIDTH, "Sudoku");
        let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
        widget_theme.apply();
        let play_board = Rc::new(RefCell::new(PlayBoard::new()));
        let control_panel = Rc::new(RefCell::new(ControlPanel::new(Rc::clone(&play_board))));
        let menu = Menu::new(Rc::clone(&play_board), WINDOW_WIDTH);

        Game {
            app: app::App::default(),
            window,
            menu,
            play_board,
            control_panel,
        }
    }

    pub fn play(&mut self) {
        self.display_game();
        self.init_window();
    }

    fn display_game(&mut self) {
        self.menu.display();
        self.play_board.borrow_mut().display();
        self.control_panel.borrow_mut().display();
    }

    fn init_window(&mut self) {
        self.window.set_color(fltk::enums::Color::White);
        self.window.make_resizable(true);
        self.window.end();
        self.window.show();
        self.app.run().unwrap();
    }
}
