use crate::components::{Damage, Health, Position, Range, Score, Speed, Target};
use hecs::World;
use rand::{thread_rng, Rng};

pub fn batch_spawn_units(world: &mut World, units: usize) {
    let mut rng = thread_rng();
    let to_spawn = (0..units).map(|_| {
        let position = Position {
            x: rng.gen_range(-1000..1000),
            y: rng.gen_range(-1000..1000),
        };
        let speed = Speed(rng.gen_range(1..5));
        let health_value: i32 = rng.gen_range(30..200);
        let health = Health {
            value: health_value,
            max: health_value,
        };
        (position, speed, health)
    });
    world.spawn_batch(to_spawn);
}

pub fn batch_spawn_towers(world: &mut World, towers: usize) {
    let mut rng = thread_rng();
    let to_spawn = (0..towers).map(|_| {
        let position = Position {
            x: rng.gen_range(-100..100),
            y: rng.gen_range(-100..100),
        };
        let damage = Damage(rng.gen_range(3..5));
        let range = Range(rng.gen_range(300..500));
        let score = Score(0);
        let target = Target { position: None };
        (position, damage, range, score, target)
    });
    world.spawn_batch(to_spawn);
}
