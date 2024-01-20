mod gui;
mod solve;
mod common;

fn main() {
    crate::gui::game::Game::new().play();
}
