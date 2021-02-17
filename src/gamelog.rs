#[derive(Default)]
pub struct GameLog {
    entries: Vec<String>,
}

impl GameLog {
    pub fn push<S>(&mut self, log: S)
    where
        S: ToString,
    {
        self.entries.push(log.to_string());
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
