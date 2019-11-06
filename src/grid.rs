use graphics::{Context, Rectangle};
use opengl_graphics::GlGraphics;

use crate::cell::Cell;
use crate::consts::{BLACK, CELL_SIZE, GREY, X_CELLS_NUM, Y_CELLS_NUM};
use crate::context::GameState;
use crate::GameContext;
use piston::input::{Button, Event, Input, UpdateArgs};

pub type Cells = [[Cell; X_CELLS_NUM]; Y_CELLS_NUM];

pub struct Grid {
    cells: Cells,
}

impl Default for Grid {
    fn default() -> Self {
        let mut cells = [[Cell::default(); X_CELLS_NUM]; Y_CELLS_NUM];
        for y in 0..Y_CELLS_NUM {
            for x in 0..X_CELLS_NUM {
                cells[y][x].position = ((x * CELL_SIZE) as f64, (y * CELL_SIZE) as f64);
            }
        }
        Grid { cells }
    }
}

impl Grid {
    pub fn update(&mut self, context: &GameContext, args: &UpdateArgs) {
        if context.state == GameState::Running {
            let mut next = self.cells;
            for y in 0..Y_CELLS_NUM {
                for x in 0..X_CELLS_NUM {
                    next[y][x].alive = self.check_neighbours(x, y);
                }
            }
            self.cells = next;
        }
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        let color = [0.5, 0.5, 0.5, 1.0];
        let radius = 0.5;
        for y in 0..Y_CELLS_NUM {
            for x in 0..X_CELLS_NUM {
                let cell = self.cells[y][x];
                let transform = ctx.transform.trans(cell.x(), cell.y());
                let rect = rectangle::square(0.0, 0.0, CELL_SIZE as f64);
                let (color, border_color) = if cell.alive {
                    (GREY, BLACK)
                } else {
                    (BLACK, GREY)
                };
                Rectangle::new_border(border_color, 0.5).color(color).draw(
                    rect,
                    &Default::default(),
                    transform,
                    gl,
                )
            }
        }
    }

    pub fn process_input(&mut self, context: &GameContext, button: Button) {
        for y in 0..Y_CELLS_NUM {
            for x in 0..X_CELLS_NUM {
                self.cells[y][x].process_input(&context, button);
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..Y_CELLS_NUM {
            for x in 0..X_CELLS_NUM {
                self.cells[y][x].alive = false;
            }
        }
    }

    pub fn neighbours_sum(&self, x: usize, y: usize) -> i32 {
        // TOP
        let state_top_left = if y > 0 && x > 0 {
            self.cells[y - 1][x - 1].alive as i32
        } else {
            0
        };

        let state_top = if y > 0 {
            self.cells[y - 1][x].alive as i32
        } else {
            0
        };

        let state_top_right = if y > 0 && x < X_CELLS_NUM - 1 {
            self.cells[y - 1][x + 1].alive as i32
        } else {
            0
        };

        // CENTER
        let state_center_left = if x > 0 {
            self.cells[y][x - 1].alive as i32
        } else {
            0
        };

        let state_center = self.cells[y][x].alive as i32;

        let state_center_right = if x < X_CELLS_NUM - 1 {
            self.cells[y][x + 1].alive as i32
        } else {
            0
        };

        // BOTTOM
        let state_bottom_left = if y < Y_CELLS_NUM - 1 && x > 0 {
            self.cells[y + 1][x - 1].alive as i32
        } else {
            0
        };

        let state_bottom = if y < Y_CELLS_NUM - 1 {
            self.cells[y + 1][x].alive as i32
        } else {
            0
        };

        let state_bottom_right = if y < Y_CELLS_NUM - 1 && x < X_CELLS_NUM - 1 {
            self.cells[y + 1][x + 1].alive as i32
        } else {
            0
        };

        state_top_left
            + state_top
            + state_top_right
            + state_center_left
            + state_center
            + state_center_right
            + state_bottom_left
            + state_bottom
            + state_bottom_right
    }

    // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    // 2. Any live cell with two or three live neighbours lives on to the next generation.
    // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn check_neighbours(&mut self, x: usize, y: usize) -> bool {
        let sum = self.neighbours_sum(x, y);
        if sum == 3 {
            true
        } else if sum == 4 {
            self.cells[y][x].alive
        } else {
            false
        }
    }

    /// TODO add save & load to help with debugging
    ///  serialize the cells to string & back
    pub fn load_grid(&mut self) {
        let gunship = vec![
            (25, 1),
            (23, 2),
            (25, 2),
            (13, 3),
            (14, 3),
            (21, 3),
            (22, 3),
            (35, 3),
            (36, 3),
            (12, 4),
            (16, 4),
            (21, 4),
            (22, 4),
            (35, 4),
            (36, 4),
            (1, 5),
            (2, 5),
            (11, 5),
            (17, 5),
            (21, 5),
            (22, 5),
            (1, 6),
            (2, 6),
            (11, 6),
            (15, 6),
            (17, 6),
            (18, 6),
            (23, 6),
            (25, 6),
            (11, 7),
            (17, 7),
            (25, 7),
            (12, 8),
            (16, 8),
            (13, 9),
            (14, 9),
        ];
        for cell in gunship {
            self.cells[cell.1][cell.0].alive = true;
        }
    }

    pub fn save_grid(&mut self) {
        let mut out = vec![];
        for y in 0..Y_CELLS_NUM {
            for x in 0..X_CELLS_NUM {
                if self.cells[y][x].alive {
                    out.push(format!("({}, {})", x, y));
                }
            }
        }

        println!("{}", out.join(", "));
    }
}
