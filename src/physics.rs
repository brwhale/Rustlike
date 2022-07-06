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
    if a.pos.y >= b.pos.y && a.pos.y <= b.pos.y+b.size {
        return AABBDirection::Down(b.pos.y+b.size - a.pos.y);
    } else if b.pos.y > a.pos.y && b.pos.y < a.pos.y+a.size {
        return AABBDirection::Up(a.pos.y+a.size - b.pos.y);
    } else {
        return AABBDirection::None;
    }
}

fn aabb_x(a: &Object, b: &Object) -> AABBDirection {
    if a.pos.x >= b.pos.x && a.pos.x <= b.pos.x+b.size {
        return AABBDirection::Left(b.pos.x+b.size - a.pos.x);
    } else if b.pos.x > a.pos.x && b.pos.x < a.pos.x+a.size {
        return AABBDirection::Right(a.pos.x+a.size - b.pos.x);
    } else {
        return AABBDirection::None;
    }
}

fn get_collide_direction(a: &Character, b: &Character) -> AABBDirection {
    let ytest = aabb_y(&a.object, &b.object);
    if ytest.is_none() {
        return AABBDirection::None;
    }
    let xtest = aabb_x(&a.object, &b.object);
    if xtest.is_none() {
        return AABBDirection::None;
    }

    if xtest.get_val() < ytest.get_val() {
        return xtest;
    } else {
        return ytest;
    }   
}

fn process_possible_collision(a: &mut Character, b: &mut Character) {
    if !a.object.velocity.is_zero() || !b.object.velocity.is_zero() {
        // adjust velocity for collisions
        match get_collide_direction(a, b) {
            AABBDirection::Left(_v) => {
                if a.object.velocity.x < 0.0 {a.object.velocity.x = 0.0}
                if b.object.velocity.x > 0.0 {b.object.velocity.x = 0.0}
            },
            AABBDirection::Right(_v) => {
                if a.object.velocity.x > 0.0 {a.object.velocity.x = 0.0}
                if b.object.velocity.x < 0.0 {b.object.velocity.x = 0.0}
            },
            AABBDirection::Down(_v) => {
                if a.object.velocity.y < 0.0 {a.object.velocity.y = 0.0}
                if b.object.velocity.y > 0.0 {b.object.velocity.y = 0.0}
            },
            AABBDirection::Up(_v) => {
                if a.object.velocity.y > 0.0 {a.object.velocity.y = 0.0}
                if b.object.velocity.y < 0.0 {b.object.velocity.y = 0.0}
            },
            _ => {},
        }  
    }
}

pub fn process(player: &mut Character, enemies: &mut Vec<Character>) {
    for enemy in enemies {
        // check collision with walls
        process_possible_collision(player, enemy);
   
        // move enemy (enemies don't colllide with each other)
        enemy.update_apply();
    }

    // finally move player
    player.update_apply();
}