use crate::utils::Vec2;

const SIZE: f64 = 50.0;
const HALF_SIZE: f64 = 0.5 * SIZE;
const DRAG_MULTIPLIER: f64 = 0.95;
const SPEED_MULTIPLIER: f64 = 20.0;

pub struct Object {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub size: f64,
}

impl Object {
    // basic constructor
    pub fn new() -> Object {
        Object{pos: Vec2::new(), velocity: Vec2::new(), size: SIZE}
    }
    
    // make the Object centered on a ertain spot
    pub fn at(p: Vec2) -> Object {
        Object{pos: p-HALF_SIZE, velocity: Vec2::new(), size: SIZE}
    }

    // per frame update
    pub fn update(&mut self, time: f64, movement: Vec2) {
        let speed = SPEED_MULTIPLIER * time;
        self.velocity += movement * speed;
    }

    pub fn apply_update(&mut self) {
        self.pos += self.velocity;
        self.velocity *= DRAG_MULTIPLIER;
    }
}