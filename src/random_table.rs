use rltk::RandomNumberGenerator;

#[derive(Clone)]
pub enum SpawnType {
    Goblin,
    Kobold,
    HealthPotion,
    FireballScroll,
    MagicMissileScroll,
    SimpleDagger,
    SimpleShield,
}

pub struct RandomEntry {
    s_type: SpawnType,
    weight: i32,
}

impl RandomEntry {
    pub fn new(s_type: SpawnType, weight: i32) -> RandomEntry {
        RandomEntry { s_type, weight }
    }
}

#[derive(Default)]
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

    pub fn insert(mut self, s_type: SpawnType, weight: i32) -> RandomTable {
        if weight > 0 {
            self.total_weight += weight;
            self.entries.push(RandomEntry::new(s_type, weight));
        }
        self
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<SpawnType> {
        if self.total_weight == 0 {
            return None;
        }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return Some(self.entries[index].s_type.clone());
            }
            roll -= self.entries[index].weight;
            index += 1;
        }

        None
    }
}
