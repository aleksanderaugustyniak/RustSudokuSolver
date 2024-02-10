use fltk::{ prelude::*, button::Button };
use fltk_theme::widget_themes;
use std::cell::RefCell;
use std::rc::Rc;
use crate::common::grid_size::GRID_SIZE;
use crate::gui::consts::*;
use crate::gui::play_board::PlayBoard;

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
            button.set_label_type(fltk::enums::LabelType::Embossed);
            button.set_frame(widget_themes::OS_BUTTON_UP_BOX);
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
            ""
        );
        self.eraser.borrow_mut().set_label_size(10);

        let panel_clone = Rc::clone(&self.panel);
        let board_clone = Rc::clone(&self.board);

        self.eraser.borrow_mut().set_callback(move |button: &mut Button| {
            board_clone.borrow_mut().set_number(&button.label());
            for but in (*panel_clone.borrow_mut()).iter_mut() {
                but.set_frame(widget_themes::OS_BUTTON_UP_BOX);
                but.redraw();
            }
            board_clone.borrow_mut().clear_highlight();
            button.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        });
    }

    fn set_panel_callbacks(&mut self) {
        for control_button in self.panel.borrow_mut().iter_mut() {
            let panel_clone = Rc::clone(&self.panel);
            let board_clone = Rc::clone(&self.board);
            let eraser_clone = Rc::clone(&self.eraser);

            control_button.set_callback(move |button: &mut Button| {
                board_clone.borrow_mut().set_number(&button.label());
                for but in (*panel_clone.borrow_mut()).iter_mut() {
                    but.set_frame(widget_themes::OS_BUTTON_UP_BOX);
                }
                button.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                board_clone.borrow_mut().highlight(&button.label());
                for but in (*panel_clone.borrow_mut()).iter_mut() {
                    but.redraw();
                }
                eraser_clone.borrow_mut().set_frame(widget_themes::OS_BUTTON_UP_BOX);
                eraser_clone.borrow_mut().redraw();
            });
        }
    }
}
