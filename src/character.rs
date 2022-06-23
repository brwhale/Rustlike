use crate::utils::Vec2;
// this is our basic character struct
// not much so far
pub struct Character {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub size: f64,
}

impl Character {
    // basic constructor
    pub fn new() -> Character {
        Character{pos: Vec2{x:0.0,y:0.0}, velocity: Vec2::new(), size: 50.0}
    }
    
    // make the character centered on a ertain spot
    pub fn at(p: Vec2) -> Character {
        Character{pos: p-25.0, velocity: Vec2::new(), size: 50.0}
    }

    // per frame update
    pub fn update(&mut self, time: f64, movement: Vec2) {
        let speed = 100.0 * time;
        self.velocity = movement * speed;
    }
}