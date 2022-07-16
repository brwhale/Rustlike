use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::*;

mod utils;
use utils::*;

mod object;
use object::*;

mod character;
use character::*;

mod level;
use level::*;

mod physics;
use physics::*;

mod render;
use render::*;

mod input;
use input::*;

// container for the game data
struct App {
    running: bool,
    player: Character,
    enemies: Vec<Character>,
    walls: Vec<Object>,
    renderer: Renderer,
    inputs: Inputs,
}

impl App {
    fn new(g: GlGraphics) -> App {
        App { 
            running: true, 
            player: Character::new(), 
            enemies: Vec::new(), 
            walls: get_wall_layout(0),
            renderer: Renderer::new(g),
            inputs: Inputs::new(),
        }
    }

    // run the game loop
    fn update(&mut self, args: &UpdateArgs) {
        if self.inputs.should_quit {
            self.running = false;
            return;
        }
        if let Some(c) = self.inputs.get_click_if_new() {
            self.enemies.push(Character::at(c));
        }
        self.player.update(args.dt, self.inputs.get_movement());

        // run physics sim
        process(&mut self.player, &mut self.enemies, &self.walls);
    }
}

// run the piston event handler loop
fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        } else if let Some(args) = e.render_args() {
            app.renderer.render(&args, &app.player, &app.enemies, &app.walls);
        } else if let Some(args) = e.press_args() {
            app.inputs.press_input(&args);
        } else if let Some(args) = e.release_args() {
            app.inputs.release_input(&args);
        }
        e.mouse_cursor(|pos| {
            app.inputs.cursor = Vec2{x:pos[0], y:pos[1]};
        });
        if !app.running {
            w.set_should_close(true);
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
