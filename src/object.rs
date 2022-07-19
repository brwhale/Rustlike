use crate::utils::Vec2;

const SIZE: f64 = 50.0;
const DRAG_MULTIPLIER: f64 = 0.92;
const SPEED_MULTIPLIER: f64 = 30.0;

pub struct Object {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub size: f64,
}

impl Object {
    // basic constructor
    pub fn new() -> Object {
        Object{pos: Vec2{x:100.0, y:100.0}, velocity: Vec2::new(), size: SIZE}
    }
    
    // make the Object centered on a ertain spot
    pub fn at(p: Vec2) -> Object {
        Object{pos: p, velocity: Vec2::new(), size: SIZE}
    }

    pub fn from(other: &Object) -> Object {
        Object{pos: other.pos, velocity: other.velocity, size: 0.0}
    }

    pub fn resize(&mut self, new_size: f64) {
        self.size = new_size;
    }

    // per frame update
    pub fn update(&mut self, time: f64, movement: Vec2) {
        let speed = SPEED_MULTIPLIER * time;
        self.velocity += movement * speed;
        self.velocity *= DRAG_MULTIPLIER;
    }

    pub fn apply_update(&mut self) {
        self.pos += self.velocity;
    }
}