pub struct UIState {
    cwd: String,
    cursor: i8,
    selected: i8,
}

pub struct UI {
    state: UIState,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            cwd: "".to_string(),
            cursor: 0,
            selected: -1,
        }
    }
}

impl UI {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
        }
    }

    pub fn render(&self) {}
}
