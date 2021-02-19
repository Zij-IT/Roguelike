use rltk::RandomNumberGenerator;

pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn insert<StrType: ToString>(mut self, name: StrType, weight: i32) -> RandomTable {
        if weight > 0 {
            self.total_weight += weight;
            self.entries.push(RandomEntry::new(name, weight));
        }
        self
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
    pub fn new<StrType: ToString>(name: StrType, weight: i32) -> RandomEntry {
        RandomEntry {
            name: name.to_string(),
            weight,
        }
    }
}
