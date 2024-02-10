use fltk::{ prelude::*, button::Button };
use fltk_theme::widget_themes;

pub fn highlight_off(button: &mut Button) {
    button.set_frame(widget_themes::OS_BUTTON_UP_BOX);
    button.redraw();
}

pub fn highlight_on(button: &mut Button) {
    button.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    button.redraw();
}
