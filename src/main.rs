use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;
use std::thread;

use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::context::GameContext;
use std::time::Duration;

mod cell;
mod consts;
mod context;
mod game;
mod grid;

fn main() {
    let opengl = OpenGL::V4_2;

    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut context = GameContext::default();

    let mut game = game::Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new().max_fps(24));

    while let Some(event) = events.next(&mut window) {
        event.mouse_cursor(|pos| context.mouse_position = pos);

        if let Some(render_args) = event.render_args() {
            game.render(&render_args);
        }

        if let Some(update_args) = event.update_args() {
            game.update(&context, &update_args);
        }

        if let Some(button) = event.press_args() {
            if let Button::Keyboard(btn) = button {
                if btn == Key::E {
                    context.toggle_state();
                }

                if btn == Key::C {
                    game.grid.clear();
                }

                if btn == Key::L {
                    game.grid.load_grid();
                }

                if btn == Key::S {
                    game.grid.save_grid();
                }
            }

            game.process_input(&context, button);
        }
    }
}
