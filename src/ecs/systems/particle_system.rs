use crate::{ParticleLifetime, Position, Renderable, Rltk};
use rltk::RGB;
use specs::prelude::*;

struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: rltk::FontCharType,
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
        fg: RGB,
        bg: RGB,
        glyph: rltk::FontCharType,
        lifetime: f32,
    ) {
        self.requests.push(ParticleRequest {
            x,
            y,
            fg,
            bg,
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
                        fg: new_particle.fg,
                        bg: new_particle.bg,
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

pub fn cull_dead_particles(ecs: &mut World, ctx: &Rltk) {
    let mut dead_particles = Vec::new();
    {
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= ctx.frame_time_ms;
            if particle.lifetime_ms < 0. {
                dead_particles.push(entity);
            }
        }
    }

    for victim in dead_particles.iter() {
        ecs.delete_entity(*victim)
            .expect("Particle not properly deleted");
    }
    ecs.maintain();
}
