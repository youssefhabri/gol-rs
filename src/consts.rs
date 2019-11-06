pub const WINDOW_WIDTH: f64 = 1000.0;
pub const WINDOW_HEIGHT: f64 = 800.0;

pub const CELL_SIZE: usize = 20;
pub const X_CELLS_NUM: usize = (WINDOW_WIDTH / CELL_SIZE as f64) as usize;
pub const Y_CELLS_NUM: usize = (WINDOW_HEIGHT / CELL_SIZE as f64) as usize;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
