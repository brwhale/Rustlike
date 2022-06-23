use crate::utils::Vec2;
use crate::object::Object;
// this is our basic character struct
// not much so far
pub struct Character {
    pub object: Object,
    // eventually there would be stats and whatever here
}

impl Character {
    // basic constructor
    pub fn new() -> Character {
        Character{object: Object::new()}
    }
    
    // make the character centered on a ertain spot
    pub fn at(p: Vec2) -> Character {
        Character{object: Object::at(p)}
    }

    // per frame update
    pub fn update(&mut self, time: f64, movement: Vec2) {
        self.object.update(time, movement);
    }

    pub fn update_apply(&mut self) {
        self.object.apply_update();
    }
}