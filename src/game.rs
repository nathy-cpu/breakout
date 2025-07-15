pub struct GameState {
    started: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self { started: false }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn started(&self) -> bool {
        self.started
    }
}
