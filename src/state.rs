pub enum InputState {
    Move,
    Edit,
}

pub enum GameState {
    Stopped,
    Running,
}

impl InputState {
    pub fn as_str(&self) -> &str {
        match self {
            InputState::Move => "MOVE",
            InputState::Edit => "EDIT",
        }
    }
}

impl GameState {
    pub fn as_str(&self) -> &str {
        match self {
            GameState::Stopped => "STOPPED",
            GameState::Running => "RUNNING",
        }
    }
}

