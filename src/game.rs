use crate::context::GameContext;
use crate::grid::Grid;
use opengl_graphics::GlGraphics;
use piston::input::{Button, RenderArgs, UpdateArgs};

pub struct Game {
    pub gl: GlGraphics,
    pub grid: Grid,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        Game {
            gl,
            grid: Grid::default(),
        }
    }
}

impl Game {
    pub fn update(&mut self, context: &GameContext, args: &UpdateArgs) {
        self.grid.update(&context, args);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let viewport = args.viewport();
        let ctx = self.gl.draw_begin(viewport);

        self.grid.render(&ctx, &mut self.gl);

        self.gl.draw_end();
    }

    pub fn process_input(&mut self, context: &GameContext, button: Button) {
        self.grid.process_input(&context, button);
    }
}
