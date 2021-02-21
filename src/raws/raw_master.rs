use super::item_structs::RawRender;
use super::Raws;
use crate::{components::*, constants::colors};
use rltk::ColorPair;
use specs::{Builder, Entity, EntityBuilder};
use std::collections::HashMap;

pub enum SpawnType {
    AtPosition(i32, i32),
}

pub struct RawMaster {
    raws: Raws,
    mob_index: HashMap<String, usize>,
    item_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> Self {
        Self {
            raws: Raws {
                mobs: Vec::new(),
                items: Vec::new(),
            },
            mob_index: HashMap::new(),
            item_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        self.item_index = HashMap::new();

        for (i, item) in self.raws.mobs.iter().enumerate() {
            self.mob_index.insert(item.name.clone(), i);
        }

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
    }

    pub fn spawn_named_entity(
        &self,
        new_entity: EntityBuilder<'_>,
        key: &str,
        pos: SpawnType,
    ) -> Option<Entity> {
        if self.item_index.contains_key(key) {
            Some(self.spawn_named_item(new_entity, self.item_index[key], pos))
        } else if self.mob_index.contains_key(key) {
            Some(self.spawn_named_mob(new_entity, self.mob_index[key], pos))
        } else {
            None
        }
    }

    pub fn spawn_named_item(
        &self,
        mut new_entity: EntityBuilder<'_>,
        index: usize,
        pos: SpawnType,
    ) -> Entity {
        let item_template = &self.raws.items[index];

        //Assign required components
        new_entity = new_entity.with(Item {});
        new_entity = new_entity.with(Name {
            name: item_template.name.clone(),
        });
        new_entity = Self::assign_render(new_entity, &item_template.render);
        new_entity = Self::assign_position(new_entity, &pos);

        //Assign optional components
        if let Some(consumable) = &item_template.consumable {
            new_entity = new_entity.with(Consumable {});
            //Assign effect components
            for effect in &consumable.effects {
                new_entity = match effect.0.as_str() {
                    "provides_healing" => new_entity.with(ProvidesHealing {
                        heal_amount: effect.1.parse().unwrap(),
                    }),
                    "range" => new_entity.with(Range {
                        range: effect.1.parse().unwrap(),
                    }),
                    "damage" => new_entity.with(InflictsDamage {
                        damage: effect.1.parse().unwrap(),
                    }),
                    "area_of_effect" => new_entity.with(AreaOfEffect {
                        radius: effect.1.parse().unwrap(),
                    }),
                    name => panic!("Consumable effect \"{}\" not implemented", name),
                }
            }
        }

        if let Some(weapon) = &item_template.weapon {
            new_entity = new_entity
                .with(MeleeDamageBonus {
                    bonus: weapon.damage_bonus,
                })
                .with(Equipable {
                    slot: EquipmentSlot::PrimaryHand,
                });
        }

        if let Some(shield) = &item_template.shield {
            new_entity = new_entity
                .with(DefenseBonus {
                    bonus: shield.defense_bonus,
                })
                .with(Equipable {
                    slot: EquipmentSlot::OffHand,
                })
        }

        new_entity.build()
    }

    pub fn spawn_named_mob(
        &self,
        mut new_entity: EntityBuilder<'_>,
        index: usize,
        pos: SpawnType,
    ) -> Entity {
        let mob_template = &self.raws.mobs[index];

        //Assign required components
        new_entity = new_entity
            .with(Monster {})
            .with(Name {
                name: mob_template.name.clone(),
            })
            .with(CombatStats {
                max_hp: mob_template.stats.max_hp,
                hp: mob_template.stats.max_hp,
                defense: mob_template.stats.defense,
                power: mob_template.stats.power,
            })
            .with(Viewshed {
                visible_tiles: vec![],
                range: mob_template.vision_range,
                is_dirty: true,
            });
        new_entity = Self::assign_render(new_entity, &mob_template.render);
        new_entity = Self::assign_position(new_entity, &pos);
        if mob_template.blocks_tile {
            new_entity = new_entity.with(BlocksTile {})
        }

        new_entity.build()
    }

    fn assign_position<'a>(new_entity: EntityBuilder<'a>, pos: &SpawnType) -> EntityBuilder<'a> {
        match pos {
            SpawnType::AtPosition(x, y) => new_entity.with(Position { x: *x, y: *y }),
        }
    }

    fn assign_render<'a>(new_entity: EntityBuilder<'a>, render: &RawRender) -> EntityBuilder<'a> {
        let colors = ColorPair::new(render.color, colors::BACKGROUND);
        new_entity.with(Render {
            glyph: render.glyph,
            render_order: render.order,
            colors,
        })
    }
}
