use crate::consts::{BLACK, CELL_SIZE, GREY, WHITE};
use crate::context::{GameContext, GameState};
use crate::grid::Cells;
use piston::input::{
    Button, Event, Input, MouseButton, MouseCursorEvent, PressEvent, RenderArgs, UpdateArgs,
};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub alive: bool,
    pub position: (f64, f64),
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            alive: false,
            position: (0.0, 0.0),
        }
    }
}

impl Cell {
    pub fn x(&self) -> f64 {
        self.position.0
    }

    pub fn y(&self) -> f64 {
        self.position.1
    }

    pub fn process_input(&mut self, context: &GameContext, button: Button) {
        if let Button::Mouse(mouse_btn) = button {
            if mouse_btn == MouseButton::Left && context.state == GameState::Paused {
                let cursor_pos = context.mouse_position;
                if cursor_pos[0] > self.x()
                    && cursor_pos[0] < self.x() + CELL_SIZE as f64
                    && cursor_pos[1] > self.y()
                    && cursor_pos[1] < self.y() + CELL_SIZE as f64
                {
                    self.alive = !self.alive;
                }
            }
        }
    }
}
