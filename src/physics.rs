use crate::object::*;
use crate::character::*;
use crate::utils::*;

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
    } else if b.pos.y >= a.pos.y && b.pos.y <= a.pos.y+a.size {
        return AABBDirection::Up(a.pos.y+a.size - b.pos.y);
    } else {
        return AABBDirection::None;
    }
}

fn aabb_x(a: &Object, b: &Object) -> AABBDirection {
    if a.pos.x >= b.pos.x && a.pos.x <= b.pos.x+b.size {
        return AABBDirection::Left(b.pos.x+b.size - a.pos.x);
    } else if b.pos.x >= a.pos.x && b.pos.x <= a.pos.x+a.size {
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

fn transfer_momentum(a: &mut Object, b: &mut Object) {
    // assume objects have equal mass for now
    let dir_mod = (a.pos - b.pos).normalized() * 0.5;
    let new_vel = mix(a.velocity, b.velocity, 0.5);
    a.velocity = new_vel + dir_mod;
    b.velocity = new_vel - dir_mod;
}

fn process_possible_spherical_collision(a: &mut Object, b: &mut Object) {
    if (a.pos - b.pos).length() < (a.size + b.size) * 0.5 {
        transfer_momentum(a, b);
    }
}

fn process_possible_static_collision(a: &mut Object, b: &Object) {
    const PUSH_OUT_AMMOUNT: f64 = 1.0;
    // adjust velocity for collisions
    match get_collide_direction(a, b) {
        AABBDirection::Left(_v) => {
            if a.velocity.x < 0.0 { a.velocity.x = 0.0; }
            a.pos.x += PUSH_OUT_AMMOUNT;
        },
        AABBDirection::Right(_v) => {
            if a.velocity.x > 0.0 { a.velocity.x = 0.0; }
            a.pos.x -= PUSH_OUT_AMMOUNT;
        },
        AABBDirection::Down(_v) => {
            if a.velocity.y < 0.0 { a.velocity.y = 0.0; }
            a.pos.y += PUSH_OUT_AMMOUNT;
        },
        AABBDirection::Up(_v) => {
            if a.velocity.y > 0.0 { a.velocity.y = 0.0; }
            a.pos.y -= PUSH_OUT_AMMOUNT;
        },
        _ => {},
    }  
}

pub fn check_visibility(start: Vec2, end: Vec2, walls: &Vec<Object>) -> bool {
    let segment_vector = end - start;
    for wall in walls {
        let test_vector = wall.pos - start;
        let segment_nearest_factor = (test_vector.dot(segment_vector) / segment_vector.dot(segment_vector)).clamp(0.0, 1.0);
        if (test_vector - segment_vector * segment_nearest_factor).length() < wall.size {
            return false;
        }
    }
    return true;
}

pub fn process(player: &mut Character, enemies: &mut Vec<Character>, walls: &Vec<Object>) {
    for i in 0..enemies.len() {
        let (_, mid_right) = enemies.split_at_mut(i);
        let (mid, right) = mid_right.split_at_mut(1);
        let enemy = &mut mid[0];

        // enemies collide with each other
        for other in right {
            process_possible_spherical_collision(&mut enemy.object, &mut other.object);
        }

        // enemies collide with player
        process_possible_spherical_collision(&mut player.object, &mut enemy.object);

        // enemies collide with walls
        for wall in &*walls {
            process_possible_static_collision(&mut enemy.object, wall);
        } 

        enemy.update_apply();
    }

    for wall in walls {
        process_possible_static_collision(&mut player.object, wall);
    }

    // finally move player
    player.update_apply();
}