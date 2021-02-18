use crate::{ParticleLifetime, Position, Renderable};
use rltk::{ColorPair, FontCharType};
use specs::prelude::*;

struct ParticleRequest {
    x: i32,
    y: i32,
    colors: ColorPair,
    glyph: FontCharType,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            requests: Vec::new(),
        }
    }

    pub fn create_particle(
        &mut self,
        x: i32,
        y: i32,
        colors: ColorPair,
        glyph: FontCharType,
        lifetime: f32,
    ) {
        self.requests.push(ParticleRequest {
            x,
            y,
            colors,
            glyph,
            lifetime,
        })
    }
}

pub struct ParticleSpawnSystem {}

impl<'a> System<'a> for ParticleSpawnSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ParticleBuilder>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, ParticleLifetime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut builder, mut positions, mut renderables, mut lifetimes) = data;
        for new_particle in builder.requests.iter() {
            let p = entities.create();
            positions
                .insert(
                    p,
                    Position {
                        x: new_particle.x,
                        y: new_particle.y,
                    },
                )
                .expect("Unable to give particle position.");
            renderables
                .insert(
                    p,
                    Renderable {
                        glyph: new_particle.glyph,
                        colors: new_particle.colors,
                        render_order: 0,
                    },
                )
                .expect("Unable to give particle renderable.");
            lifetimes
                .insert(
                    p,
                    ParticleLifetime {
                        lifetime_ms: new_particle.lifetime,
                    },
                )
                .expect("Unable to give particle lifetime.");
        }
        builder.requests.clear();
    }
}

pub fn cull_dead_particles(ecs: &mut World, frame_time: f32) {
    let mut particles = ecs.write_storage::<ParticleLifetime>();
    let entities = ecs.entities();

    let mut dead_particles = Vec::new();

    for (ent, particle) in (&entities, &mut particles).join() {
        particle.lifetime_ms -= frame_time;
        if particle.lifetime_ms <= 0. {
            dead_particles.push(ent);
        }
    }

    std::mem::drop(particles);
    std::mem::drop(entities);

    for victim in dead_particles.iter() {
        ecs.delete_entity(*victim)
            .expect("Particle not properly deleted");
    }

    ecs.maintain();
}
