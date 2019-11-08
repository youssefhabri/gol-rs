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

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }
}
