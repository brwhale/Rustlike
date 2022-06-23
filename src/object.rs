use crate::utils::Vec2;

pub struct Object {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub size: f64,
    //pub textureId: i32,
}

impl Object {
    // basic constructor
    pub fn new() -> Object {
        Object{pos: Vec2{x:0.0,y:0.0}, velocity: Vec2::new(), size: 50.0}
    }
    
    // make the Object centered on a ertain spot
    pub fn at(p: Vec2) -> Object {
        Object{pos: p-25.0, velocity: Vec2::new(), size: 50.0}
    }

    // per frame update
    pub fn update(&mut self, time: f64, movement: Vec2) {
        let speed = 100.0 * time;
        self.velocity = movement * speed;
    }

    pub fn apply_update(&mut self) {
        self.pos += self.velocity;
        self.velocity = Vec2::new();
    }
}