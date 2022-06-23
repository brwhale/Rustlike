use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{Graphics, Button, PistonWindow, EventSettings, RenderEvent, UpdateEvent, Events, UpdateArgs, WindowSettings, RenderArgs};
use piston::input::*;

mod utils;
use utils::*;

mod drawing;
use drawing::*;

mod character;
use character::*;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// container for the game data
struct App {
    pub(crate) gl: GlGraphics,

    running: bool,
    keys: Map<Key, bool>,
    player: Character,
    enemies: Vec<Character>,
    cursor: Vec2,
}

impl App {
    fn new(g: GlGraphics) -> App {
        App { 
            gl: g, 
            running: true, 
            keys: Map::new(), 
            player: Character::new(), 
            enemies: Vec::new(), 
            cursor: Vec2::new() 
        }
    }

    // our main drawing function
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
            // draw the enemies
            for e in &self.enemies {
                g.draw_rectangle([
                    e.pos.x, 
                    e.pos.y,
                    e.size,
                    e.size
                    ], RED, &c);
            }
            // draw us
            g.draw_rectangle([
                self.player.pos.x, 
                self.player.pos.y, 
                self.player.size,
                self.player.size
                ], WHITE, &c);
        });
    }

    // run the game loop
    fn update(&mut self, args: &UpdateArgs) { 
        let mut movement = Vec2::new();
        if *self.keys.index(Key::S) {
            movement.y += 1.0;
        }
        if *self.keys.index(Key::W) {
            movement.y -= 1.0;
        }
        if *self.keys.index(Key::A) {
            movement.x -= 1.0;
        }
        if *self.keys.index(Key::D) {
            movement.x += 1.0;
        }
        movement.normalize();
        self.player.update(args.dt, movement);
    }

    // event handler for presses
    fn press_input(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => { 
                *self.keys.index(*key) = true;
                // quit on mac on command + Q
                if *key == Key::Q && *self.keys.index(Key::Unknown) {
                    self.running = false;
                }
            },
            Button::Mouse(button) => println!("Pressed mouse button '{:?}'", button),
            _ => {}
        }
    }

    // event handler for releases
    fn release_input(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => *self.keys.index(*key) = false,
            Button::Mouse(button) => {
                println!("Released mouse button '{:?}'", button);
                self.enemies.push(Character::at(self.cursor));
            },
            _ => {}
        }
    }
}

// run the piston event handler loop
fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        } else if let Some(args) = e.render_args() {
            app.render(&args);
        } else if let Some(args) = e.press_args() {
            app.press_input(&args);
        } else if let Some(args) = e.release_args() {
            app.release_input(&args);
        }
        e.mouse_cursor(|pos| {
            app.cursor = Vec2{x:pos[0], y:pos[1]};
        });
        if !app.running {
            break;
        }
    }
}

// after all that, here's our entry point!
fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new("roguelike", [1280, 720])
        .graphics_api(opengl)
        .build()
        .unwrap();
        
    run_loop(&mut App::new(GlGraphics::new(opengl)), &mut window);
}
