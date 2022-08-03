pub struct State {
    pub cwd: String,
    pub cursor: i8,
    pub selected: i8,
}

impl State {
    pub fn new() -> Self {
        Self {
            cwd: "".to_string(),
            cursor: 0,
            selected: -1,
        }
    }
}
