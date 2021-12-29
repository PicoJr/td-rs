use crate::components::{Damage, Health, Position, Range, Score, Speed, Target, Waypoint};
use crate::systems::manhattan_distance;
use hecs::{With, World};
use macroquad::prelude::Vec2;
use rand::{thread_rng, Rng};

pub fn batch_spawn_units(world: &mut World, units: usize, spawn_position: &Position) {
    let mut rng = thread_rng();
    let to_spawn = (0..units).map(|_| {
        let position = Position {
            x: spawn_position.x + rng.gen_range(-1000..1000),
            y: spawn_position.y + rng.gen_range(-1000..1000),
        };
        let speed = Speed(rng.gen_range(1..5));
        let health_value: i32 = rng.gen_range(30..200);
        let health = Health {
            value: health_value,
            max: health_value,
        };
        let waypoint = Waypoint { index: 0 };
        (position, speed, health, waypoint)
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

pub fn spawn_tower(world: &mut World, position: &Vec2) {
    let mut rng = thread_rng();
    let position = Position {
        x: position.x as i32,
        y: position.y as i32,
    };
    let damage = Damage(rng.gen_range(3..5));
    let range = Range(rng.gen_range(300..500));
    let score = Score(0);
    let target = Target { position: None };

    world.spawn((position, damage, range, score, target));
}

pub fn remove_tower(world: &mut World, position: &Vec2) {
    let remove_position = Position {
        x: position.x as i32,
        y: position.y as i32,
    };
    let closest_entity_to_position = world
        .query::<With<Damage, &Position>>()
        .iter()
        .filter(|(_id, p)| manhattan_distance(p, &remove_position) < 10i32)
        .min_by_key(|(_id, p)| manhattan_distance(p, &remove_position))
        .map(|(id, _p)| id);
    if let Some(id) = closest_entity_to_position {
        world.despawn(id).unwrap();
    }
}

pub fn closest_tower(world: &World, position: &Vec2) -> Option<(Vec2, f32)> {
    let target = Position {
        x: position.x as i32,
        y: position.y as i32,
    };
    let closest_entity_to_position = world
        .query::<With<Damage, (&Position, &Range)>>()
        .iter()
        .filter(|(_id, (p, _range))| manhattan_distance(p, &target) < 10i32)
        .min_by_key(|(_id, (p, _range))| manhattan_distance(p, &target))
        .map(|(_id, (p, r))| (p.x, p.y, r.0));
    closest_entity_to_position.map(|(px, py, r)| (Vec2::new(px as f32, py as f32), r as f32))
}
