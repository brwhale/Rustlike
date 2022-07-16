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
    pub(crate) gl: GlGraphics,
    texture: Texture,
    enemy_texture: Texture,
    wall_texture: Texture,
}

impl Renderer {
    pub fn new(g: GlGraphics) -> Renderer {
        Renderer { 
            gl: g,
            texture: get_texture("Resources/player.png"),
            enemy_texture: get_texture("Resources/enemy.png"),
            wall_texture: get_texture("Resources/leaf_wall.png"),
        }
    }

    // our main drawing function
    pub fn render(&mut self, args: &RenderArgs, player: &Character, enemies: &Vec<Character>, walls: &Vec<Object>) {
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
            // draw the enemies
            for w in walls {
                Image::new()
                    .rect(graphics::rectangle::square(w.pos.x, w.pos.y, w.size))
                    .draw(&self.wall_texture, &graphics::DrawState::default(), c.transform, g);
            }
            // draw the enemies
            for e in enemies {
                Image::new()
                    .rect(graphics::rectangle::square(e.object.pos.x, e.object.pos.y, e.object.size))
                    .draw(&self.enemy_texture, &graphics::DrawState::default(), c.transform, g);
            }
            // draw us
            Image::new()
                .rect(graphics::rectangle::square(player.object.pos.x, player.object.pos.y, player.object.size))
                .draw(&self.texture, &graphics::DrawState::default(), c.transform, g);
        });
    }
}