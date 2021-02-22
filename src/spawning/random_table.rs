use rltk::RandomNumberGenerator;

pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn insert<S: ToString + std::fmt::Display>(&mut self, name: &S, weight: i32) {
        if weight > 0 {
            self.total_weight += weight;
            self.entries.push(RandomEntry::new(name, weight));
        }
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<String> {
        if self.total_weight == 0 {
            return None;
        }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return Some(self.entries[index].name.clone());
            }
            roll -= self.entries[index].weight;
            index += 1;
        }

        None
    }
}

struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new<S: ToString + std::fmt::Display>(name: &S, weight: i32) -> Self {
        Self {
            name: name.to_string(),
            weight,
        }
    }
}
