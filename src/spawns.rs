use crate::components::{Damage, Health, Position, Range, Score, Speed};
use hecs::World;
use rand::{thread_rng, Rng};

pub fn batch_spawn_units(world: &mut World, units: usize) {
    let mut rng = thread_rng();
    let to_spawn = (0..units).map(|_| {
        let position = Position {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
        };
        let speed = Speed(rng.gen_range(1..5));
        let health = Health(rng.gen_range(30..50));
        (position, speed, health)
    });
    world.spawn_batch(to_spawn);
}

pub fn batch_spawn_towers(world: &mut World, towers: usize) {
    let mut rng = thread_rng();
    let to_spawn = (0..towers).map(|_| {
        let position = Position {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
        };
        let damage = Damage(rng.gen_range(30..50));
        let range = Range(rng.gen_range(5..10));
        let score = Score(0);
        (position, damage, range, score)
    });
    world.spawn_batch(to_spawn);
}
