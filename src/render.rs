use opengl_graphics::{GlGraphics, Texture};
use piston_window::*;

use crate::{Character, object::Object};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn get_texture(path: &str) -> Texture {
    Texture::from_path(
        std::path::Path::new(path),
        &TextureSettings::new()
    ).unwrap()
}

pub struct Renderer {
    gl: GlGraphics,
    texture: Texture,
    enemy_texture: Texture,
    wall_texture: Texture,
    attack_texture: Texture,
}

impl Renderer {
    pub fn new(g: GlGraphics) -> Renderer {
        Renderer { 
            gl: g,
            texture: get_texture("Resources/player.png"),
            enemy_texture: get_texture("Resources/enemy.png"),
            wall_texture: get_texture("Resources/leaf_wall.png"),
            attack_texture: get_texture("Resources/attack.png"),
        }
    }

    // helper to draw  individual objects
    fn draw(texture: &Texture, obj: &Object, c: Context, g: &mut GlGraphics) {
        let half_size = obj.size * 0.5;
        Image::new()
            .rect(graphics::rectangle::square(obj.pos.x - half_size, obj.pos.y - half_size, obj.size))
            .draw(texture, &graphics::DrawState::default(), c.transform, g);
    }

    // our main drawing function
    pub fn render(&mut self, args: &RenderArgs, player: &Character, enemies: &Vec<Character>, walls: &Vec<Object>, attacks: &Vec<Object>) {
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
            // draw the enemies
            for w in walls {
                Renderer::draw(&self.wall_texture, w, c, g);
            }
            // draw the enemies
            for e in enemies {
                Renderer::draw(&self.enemy_texture, &e.object, c, g);
            }
            // draw any attacks
            for a in attacks {
                Renderer::draw(&self.attack_texture, a, c, g);
            }
            // draw us
            Renderer::draw(&self.texture, &player.object, c, g);
        });
    }
}