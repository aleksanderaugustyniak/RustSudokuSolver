use fltk::{ prelude::*, button::* };
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::board::GRID_SIZE;
use crate::gui::colors::*;
use crate::gui::play_board::PlayBoard;

const MENU_WIDTH: i32 = 25;
const BUTTON_SIZE: i32 = 50;
const BOARD_OFFSET_LEFT: i32 = 30;
const BOARD_OFFSET_TOP: i32 = 2 * MENU_WIDTH;
const TOP_OFFSET: i32 = BUTTON_SIZE * (GRID_SIZE as i32) + BOARD_OFFSET_TOP + 25;

pub struct ControlPanel {
    panel: Rc<RefCell<[Button; GRID_SIZE]>>,
    board: Rc<RefCell<PlayBoard>>,
    eraser: Rc<RefCell<Button>>,
}

impl ControlPanel {
    pub fn new(play_board: Rc<RefCell<PlayBoard>>) -> Self {
        ControlPanel {
            panel: Default::default(),
            board: Rc::clone(&play_board),
            eraser: Default::default(),
        }
    }

    pub fn display(&mut self) {
        for (number, button) in self.panel.borrow_mut().iter_mut().enumerate() {
            *button = Button::new(
                BOARD_OFFSET_LEFT + (number as i32) * BUTTON_SIZE,
                TOP_OFFSET,
                BUTTON_SIZE,
                BUTTON_SIZE,
                ""
            );
            button.set_label(&format!("{}", number + 1));
            button.set_label_size(22);
            button.set_color(LIGHT_BUTTON_COLOR);
        }

        self.display_eraser();
        self.set_panel_callbacks();
    }

    fn display_eraser(&mut self) {
        *self.eraser.borrow_mut() = Button::new(
            BOARD_OFFSET_LEFT + (GRID_SIZE as i32) * BUTTON_SIZE,
            TOP_OFFSET,
            BUTTON_SIZE,
            BUTTON_SIZE,
            "123\n456\n789"
        );
        self.eraser.borrow_mut().set_label_size(10);

        let panel_clone = Rc::clone(&self.panel);
        let board_clone = Rc::clone(&self.board);

        self.eraser.borrow_mut().set_callback(move |button: &mut Button| {
            board_clone.borrow_mut().set_number(&button.label().to_string());
            for but in (*panel_clone.borrow_mut()).iter_mut() {
                but.set_color(LIGHT_BUTTON_COLOR);
                but.redraw();
            }
            board_clone.borrow_mut().clear_color();
            button.set_color(HIGHLIGHTED_BUTTON_COLOR);
        });
    }

    fn set_panel_callbacks(&mut self) {
        for control_button in self.panel.borrow_mut().iter_mut() {
            let panel_clone = Rc::clone(&self.panel);
            let board_clone = Rc::clone(&self.board);
            let eraser_clone = Rc::clone(&self.eraser);

            control_button.set_callback(move |button: &mut Button| {
                board_clone.borrow_mut().set_number(&button.label().to_string());
                for but in (*panel_clone.borrow_mut()).iter_mut() {
                    but.set_color(LIGHT_BUTTON_COLOR);
                    but.redraw();
                }
                board_clone.borrow_mut().highlight(&button.label());
                button.set_color(HIGHLIGHTED_BUTTON_COLOR);
                eraser_clone.borrow_mut().set_color(LIGHT_BUTTON_COLOR);
                eraser_clone.borrow_mut().redraw();
            });
        }
    }
}
