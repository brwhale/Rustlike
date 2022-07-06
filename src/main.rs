use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::*;

mod utils;
use utils::*;

mod object;

mod character;
use character::*;

mod physics;
use physics::*;

mod render;
use render::*;

// container for the game data
struct App {
    running: bool,
    keys: Map<Key, bool>,
    player: Character,
    enemies: Vec<Character>,
    cursor: Vec2,
    renderer: Renderer,
}

impl App {
    fn new(g: GlGraphics) -> App {
        App { 
            running: true, 
            keys: Map::new(), 
            player: Character::new(), 
            enemies: Vec::new(), 
            cursor: Vec2::new(),
            renderer: Renderer::new(g),
        }
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

        // run physics sim
        process(&mut self.player, &mut self.enemies);
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
            //Button::Mouse(button) => {},
            _ => {}
        }
    }

    // event handler for releases
    fn release_input(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => *self.keys.index(*key) = false,
            Button::Mouse(_button) => {
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
            app.renderer.render(&args, &app.player, &app.enemies);
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
