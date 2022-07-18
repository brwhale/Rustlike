use crate::{object::Object, utils::Vec2, character::Character};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

fn draw_line(start: Vec2, end: Vec2, objects: &mut Vec<Object>) {
    let vector = end - start;
    let length = vector.length();
    let angle = vector.normalized();
    let mut steps = 0.0;
    while steps <= length {
        objects.push(Object::at(start + angle * steps));
        steps += 50.0;
    }
}

pub fn get_wall_layout(level: u64) -> Vec<Object> {
    let mut vec = Vec::new();
    draw_line(Vec2{x: 0.0, y: 0.0}, Vec2{x: 0.0, y: 720.0}, &mut vec);
    draw_line(Vec2{x: 1280.0, y: 0.0}, Vec2{x: 1280.0, y: 720.0}, &mut vec);
    draw_line(Vec2{x: 0.0, y: 0.0}, Vec2{x: 1280.0, y: 0.0}, &mut vec);
    draw_line(Vec2{x: 0.0, y: 720.0}, Vec2{x: 1280.0, y: 720.0}, &mut vec);

    let mut rng = StdRng::seed_from_u64(level + 123812925);
    for _ in 0..level {
        let pos = Vec2{x: rng.gen::<f64>() * 980.0 + 150.0, y: rng.gen::<f64>() * 420.0 + 150.0};
        let angle = Vec2{x: rng.gen::<f64>() * 2.0 - 1.0, y: rng.gen::<f64>() * 2.0 - 1.0}.normalized();
        let distance = rng.gen::<f64>() * 700.0;
        draw_line(pos, pos + angle * distance, &mut vec);
    }

    return vec;
}

pub fn get_enemy_layout(level: u64) -> Vec<Character> {
    let mut vec = Vec::new();

    let mut rng = StdRng::seed_from_u64(level + 1812925);
    for _ in 0..level {
        let pos = Vec2{x: rng.gen::<f64>() * 980.0 + 150.0, y: rng.gen::<f64>() * 420.0 + 150.0};
        vec.push(Character::at(pos));
    }

    return vec;
}