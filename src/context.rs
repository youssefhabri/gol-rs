#[derive(Eq, PartialEq)]
pub enum GameState {
    Paused,
    Running,
}

pub struct GameContext {
    pub state: GameState,
    pub mouse_position: [f64; 2],
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            state: GameState::Paused,
            mouse_position: [0.0; 2],
        }
    }
}

impl GameContext {
    pub fn toggle_state(&mut self) {
        if self.state == GameState::Paused {
            self.state = GameState::Running
        } else {
            self.state = GameState::Paused
        }
    }
}
