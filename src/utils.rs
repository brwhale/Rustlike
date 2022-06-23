use std::{collections::HashMap, ops};

// basic 2d vector type
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2{x: 0.0, y: 0.0}
    }

    pub fn normalize(&mut self) {
        let len = (self.x*self.x + self.y*self.y).sqrt();
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, other: f64) -> Vec2 {
        Vec2 { x: self.x + other, y: self.y + other }
    }
}

impl ops::AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Sub<f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: f64) -> Vec2 {
        Vec2 { x: self.x - other, y: self.y - other }
    }
}

impl ops::SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
    }
}

impl ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x * other.x, y: self.y * other.y }
    }
}

impl ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f64) -> Vec2 {
        Vec2 { x: self.x * other, y: self.y * other }
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

// more convenient map api
pub struct Map<K: Eq + std::hash::Hash, V: Default> {
    map: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V: Default> Map<K, V> {
    pub fn new() -> Map<K, V>{
        Map{map: HashMap::new()}
    }
    pub fn index(&mut self, k: K) -> &mut V {
        self.map.entry(k).or_default()
    }
}