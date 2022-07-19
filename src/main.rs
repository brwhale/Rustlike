use std::time::Instant;
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{PistonWindow, Window, AdvancedWindow, UpdateArgs, UpdateEvent, PressEvent, ReleaseEvent, RenderEvent, MouseCursorEvent, Events, EventSettings, WindowSettings};

mod utils;
use utils::Vec2;

mod object;
use object::Object;

mod character;
use character::Character;

mod level;
use level::{get_enemy_layout, get_wall_layout};

mod physics;
use physics::{process_physics, check_visibility};

mod render;
use render::Renderer;

mod input;
use input::Inputs;

// container for the game data
struct App {
    running: bool,
    player: Character,
    enemies: Vec<Character>,
    walls: Vec<Object>,
    attacks: Vec<Object>,
    renderer: Renderer,
    inputs: Inputs,
    current_level: u64,
}

impl App {
    fn new(g: GlGraphics) -> App {
        App { 
            running: true, 
            player: Character::new(), 
            enemies: get_enemy_layout(1), 
            walls: get_wall_layout(1),
            attacks: Vec::new(),
            renderer: Renderer::new(g),
            inputs: Inputs::new(),
            current_level: 1,
        }
    }

    // run the game loop
    fn update(&mut self, args: &UpdateArgs) {
        if self.inputs.should_quit {
            self.running = false;
            return;
        }

        // reset level if requested
        if self.inputs.should_load() {
            self.current_level += 1;
            self.enemies = get_enemy_layout(self.current_level);
            self.walls = get_wall_layout(self.current_level);
            self.attacks = Vec::new();
        }

        // click to place extra enemies
        if let Some(c) = self.inputs.get_click_if_new() {
            self.enemies.push(Character::at(c));
        }

        // update player
        self.player.update(args.dt, self.inputs.get_movement());
        if self.inputs.get_is_attacking() {
            self.attacks.push(Object::from(&self.player.object));
        }

        // update any active attacks
        for attack in &mut self.attacks {
            attack.resize(attack.size + 150.0 * args.dt);
        }
        self.attacks.retain(|a| a.size < 150.0);

        // update enemies
        for enemy in &mut self.enemies {
            let lookat = self.player.object.pos - enemy.object.pos;
            let distance = lookat.length();
            if 50.0 < distance && distance < 1000.0 && check_visibility(self.player.object.pos, enemy.object.pos, &self.walls) {
                enemy.update(args.dt, lookat.normalized());
            } else {
                enemy.update(args.dt, Vec2::new());
            }
        }

        // run physics sim
        process_physics(&mut self.player, &mut self.enemies, &self.walls, &mut self.attacks);
    }
}

// run the piston event handler loop
fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings {
        max_fps: 120,
        ups: 120,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
        ups_reset: 0,
    });
    let mut title_update_time = Instant::now();
    let mut update_frames: u64 = 0;
    let mut render_frames: u64 = 0;
    while let Some(e) = events.next(w) {
        if let Some(args) = e.update_args() {
            if title_update_time.elapsed().as_secs_f64() >= 1.0 {
                w.set_title(format!("rougelike {} fps {} ups", render_frames, update_frames));
                title_update_time = Instant::now();
                update_frames = 0;
                render_frames = 0;
            } else {
                update_frames += 1;
            }
            app.update(&args);
        } else if let Some(args) = e.render_args() {
            render_frames += 1;
            app.renderer.render(&args, &app.player, &app.enemies, &app.walls, &app.attacks);
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
