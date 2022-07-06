use crate::{utils::*};
use piston_window::{Key, Button};
pub struct Inputs {
    keys: Map<Key, bool>,
    pub cursor: Vec2,
    pub should_quit: bool,
    mouse_down: bool,
    mouse_was_down: bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs { 
            keys: Map::new(), 
            cursor: Vec2::new(),
            should_quit: false,
            mouse_down: false,
            mouse_was_down: false,
        }
    }

    pub fn get_click_if_new(&mut self) -> Option<Vec2> {
        let was_up = !self.mouse_was_down;
        self.mouse_was_down = self.mouse_down;
        if self.mouse_down && was_up {
            return Some(self.cursor);
        }
        return None;
    }

    pub fn get_movement(&mut self) -> Vec2 {
        let mut movement = Vec2::new();
        if *self.keys.index(Key::S) {
            movement.y += 1.0;
        }
        if *self.keys.index(Key::W) {
            movement.y -= 1.0;
        }
        if *self.keys.index(Key::A) {
            movement.x -= 1.0;
        }
        if *self.keys.index(Key::D) {
            movement.x += 1.0;
        }
        movement.normalize();
        return movement;
    }

    // event handler for presses
    pub fn press_input(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => { 
                *self.keys.index(*key) = true;
                // quit on mac on command + Q
                if *key == Key::Q && *self.keys.index(Key::Unknown) {
                    self.should_quit = true;
                }
            },
            Button::Mouse(_button) => {
                self.mouse_down = true;
            },
            _ => {}
        }
    }

    // event handler for releases
    pub fn release_input(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => *self.keys.index(*key) = false,
            Button::Mouse(_button) => {
                self.mouse_down = false;
            },
            _ => {}
        }
    }
}