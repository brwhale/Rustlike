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

fn aabb_y(a: &Character, b: &Character) -> AABBDirection {
    if a.pos.y >= b.pos.y && a.pos.y <= b.pos.y+b.size {
        return AABBDirection::Down(b.pos.y+b.size - a.pos.y);
    } else if b.pos.y > a.pos.y && b.pos.y < a.pos.y+a.size {
        return AABBDirection::Up(a.pos.y+a.size - b.pos.y);
    } else {
        return AABBDirection::None;
    }
}

fn aabb_x(a: &Character, b: &Character) -> AABBDirection {
    if a.pos.x >= b.pos.x && a.pos.x <= b.pos.x+b.size {
        return AABBDirection::Left(b.pos.x+b.size - a.pos.x);
    } else if b.pos.x > a.pos.x && b.pos.x < a.pos.x+a.size {
        return AABBDirection::Right(a.pos.x+a.size - b.pos.x);
    } else {
        return AABBDirection::None;
    }
}

pub fn process(player: &mut Character, enemies: &mut Vec<Character>) {
    for mut enemy in enemies {
        // move enemy (enemies don't colllide with each other)
        enemy.pos += enemy.velocity;
        enemy.velocity = Vec2::new();

        // early out if player isn't moving
        if player.velocity.is_zero() {
            continue;
        }
        // early out if not colliding
        let ytest = aabb_y(player, enemy);
        if ytest.is_none() {
            continue;
        }
        let xtest = aabb_x(player, enemy);
        if xtest.is_none() {
            continue;
        }
        // adjust velocity for collisions
        if xtest.get_val() < ytest.get_val() {
            match xtest {
                AABBDirection::Left(_v) => if player.velocity.x < 0.0 {player.velocity.x = 0.0},
                AABBDirection::Right(_v) => if player.velocity.x > 0.0 {player.velocity.x = 0.0},
                _ => {},
            }
        } else {
            match ytest {
                AABBDirection::Down(_v) => if player.velocity.y < 0.0 {player.velocity.y = 0.0},
                AABBDirection::Up(_v) => if player.velocity.y > 0.0 {player.velocity.y = 0.0},
                _ => {},
            }
        }    
    }

    // finally move player
    player.pos += player.velocity;
    player.velocity = Vec2::new();
}