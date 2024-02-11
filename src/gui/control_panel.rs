use fltk::{ prelude::*, button::Button };
use std::cell::RefCell;
use std::rc::Rc;
use crate::common::grid_size::GRID_SIZE;
use crate::gui::button::*;
use crate::gui::consts::*;
use crate::gui::play_board::PlayBoard;

const TOP_OFFSET: i32 = BUTTON_SIZE * (GRID_SIZE as i32) + BOARD_OFFSET_TOP + 25;
type Panel = [Button; GRID_SIZE];

pub struct ControlPanel {
    panel: Rc<RefCell<Panel>>,
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
            *button = Self::create_button(number);
            button.set_label(&format!("{}", number + 1));
            button.set_label_size(22);
            button.set_label_type(fltk::enums::LabelType::Embossed);
            highlight_off(button);
        }

        self.display_eraser();
        self.set_panel_callbacks();
    }

    fn display_eraser(&mut self) {
        *self.eraser.borrow_mut() = Self::create_button(GRID_SIZE);
        self.eraser.borrow_mut().set_label_size(10);

        let panel_clone = Rc::clone(&self.panel);
        let board_clone = Rc::clone(&self.board);

        self.eraser.borrow_mut().set_callback(move |button: &mut Button| {
            board_clone.borrow_mut().set_number(&button.label());
            Self::clear_highlight(&mut panel_clone.borrow_mut());
            board_clone.borrow_mut().clear_highlight();
            highlight_on(button);
        });
    }

    fn set_panel_callbacks(&mut self) {
        for control_button in self.panel.borrow_mut().iter_mut() {
            let panel_clone = Rc::clone(&self.panel);
            let board_clone = Rc::clone(&self.board);
            let eraser_clone = Rc::clone(&self.eraser);

            control_button.set_callback(move |button: &mut Button| {
                board_clone.borrow_mut().set_number(&button.label());
                Self::clear_highlight(&mut panel_clone.borrow_mut());
                highlight_on(button);
                board_clone.borrow_mut().highlight(&button.label());
                highlight_off(&mut eraser_clone.borrow_mut());
            });
        }
    }

    fn clear_highlight(control_panel: &mut Panel) {
        for button in control_panel.iter_mut() {
            highlight_off(button);
        }
    }

    fn create_button(index: usize) -> Button {
        let left_offset = BOARD_OFFSET_LEFT + (index as i32) * BUTTON_SIZE;
        Button::new(left_offset, TOP_OFFSET, BUTTON_SIZE, BUTTON_SIZE, "")
    }
}
