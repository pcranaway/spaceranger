use super::state::State;

pub struct UI {
    pub state: State,
}

impl UI {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn render(&self) {}
}
