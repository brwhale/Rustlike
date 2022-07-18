use crate::{object::Object, utils::Vec2};

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

pub fn get_wall_layout(_level: u8) -> Vec<Object> {
    let mut vec = Vec::new();
    draw_line(Vec2{x: 0.0, y: 0.0}, Vec2{x: 0.0, y: 900.0}, &mut vec);
    draw_line(Vec2{x: 200.0, y: 200.0}, Vec2{x: 200.0, y: 400.0}, &mut vec);
    draw_line(Vec2{x: 200.0, y: 400.0}, Vec2{x: 400.0, y: 400.0}, &mut vec);
    draw_line(Vec2{x: 300.0, y: 300.0}, Vec2{x: 600.0, y: 400.0}, &mut vec);
    return vec;
}