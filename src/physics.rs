use crate::object::*;
use crate::character::*;
enum AABBDirection {
    None,
    Left(f64),
    Right(f64),
    Up(f64),
    Down(f64),
}

impl AABBDirection {
    fn is_none(&self) -> bool {
        match *self {
            AABBDirection::None => true,
            _ => false,
        }
    }

    fn get_val(&self) -> f64 {
        match *self {
            AABBDirection::None => 0.0,
            AABBDirection::Left(v) => v,
            AABBDirection::Right(v) => v,
            AABBDirection::Up(v) => v,
            AABBDirection::Down(v) => v,
        }
    }
}

fn aabb_y(a: &Object, b: &Object) -> AABBDirection {
    if a.pos.y > b.pos.y && a.pos.y < b.pos.y+b.size {
        return AABBDirection::Down(b.pos.y+b.size - a.pos.y);
    } else if b.pos.y > a.pos.y && b.pos.y < a.pos.y+a.size {
        return AABBDirection::Up(a.pos.y+a.size - b.pos.y);
    } else {
        return AABBDirection::None;
    }
}

fn aabb_x(a: &Object, b: &Object) -> AABBDirection {
    if a.pos.x > b.pos.x && a.pos.x < b.pos.x+b.size {
        return AABBDirection::Left(b.pos.x+b.size - a.pos.x);
    } else if b.pos.x > a.pos.x && b.pos.x < a.pos.x+a.size {
        return AABBDirection::Right(a.pos.x+a.size - b.pos.x);
    } else {
        return AABBDirection::None;
    }
}

fn get_collide_direction(a: &Object, b: &Object) -> AABBDirection {
    let ytest = aabb_y(&a, &b);
    if ytest.is_none() {
        return AABBDirection::None;
    }
    let xtest = aabb_x(&a, &b);
    if xtest.is_none() {
        return AABBDirection::None;
    }

    if xtest.get_val() < ytest.get_val() {
        return xtest;
    } else {
        return ytest;
    }   
}

fn process_possible_collision(a: &mut Object, b: &mut Object) {
    if !a.velocity.is_zero() || !b.velocity.is_zero() {
        // adjust velocity for collisions
        match get_collide_direction(a, b) {
            AABBDirection::Left(_v) => {
                if a.velocity.x < 0.0 {a.velocity.x = 0.0}
                if b.velocity.x > 0.0 {b.velocity.x = 0.0}
            },
            AABBDirection::Right(_v) => {
                if a.velocity.x > 0.0 {a.velocity.x = 0.0}
                if b.velocity.x < 0.0 {b.velocity.x = 0.0}
            },
            AABBDirection::Down(_v) => {
                if a.velocity.y < 0.0 {a.velocity.y = 0.0}
                if b.velocity.y > 0.0 {b.velocity.y = 0.0}
            },
            AABBDirection::Up(_v) => {
                if a.velocity.y > 0.0 {a.velocity.y = 0.0}
                if b.velocity.y < 0.0 {b.velocity.y = 0.0}
            },
            _ => {},
        }  
    }
}

fn process_possible_static_collision(a: &mut Object, b: &Object) {
    const PUSH_OUT_AMMOUNT: f64 = 0.1;
    if !a.velocity.is_zero() || !b.velocity.is_zero() {
        // adjust velocity for collisions
        match get_collide_direction(a, b) {
            AABBDirection::Left(_v) => {
                if a.velocity.x < 0.0 {a.velocity.x = 0.0; a.pos.x += PUSH_OUT_AMMOUNT;}
            },
            AABBDirection::Right(_v) => {
                if a.velocity.x > 0.0 {a.velocity.x = 0.0; a.pos.x -= PUSH_OUT_AMMOUNT;}
            },
            AABBDirection::Down(_v) => {
                if a.velocity.y < 0.0 {a.velocity.y = 0.0; a.pos.y += PUSH_OUT_AMMOUNT;}
            },
            AABBDirection::Up(_v) => {
                if a.velocity.y > 0.0 {a.velocity.y = 0.0; a.pos.y -= PUSH_OUT_AMMOUNT;}
            },
            _ => {},
        }  
    }
}

pub fn process(player: &mut Character, enemies: &mut Vec<Character>, walls: &Vec<Object>) {
    for wall in walls {
        // check collision with walls
        process_possible_static_collision(&mut player.object, wall);
        for enemy in &mut *enemies {
            process_possible_static_collision(&mut enemy.object, wall);
        }
    }

    for enemy in enemies {
        // check collision with enemies
        process_possible_collision(&mut player.object, &mut enemy.object);
   
        // move enemy (enemies don't colllide with each other)
        enemy.update_apply();
    }

    // finally move player
    player.update_apply();
}