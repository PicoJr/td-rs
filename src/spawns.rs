use crate::components::{Damage, Health, Position, Range, Score, Speed, Target, Waypoint};
use hecs::{Entity, With, World};
use macroquad::prelude::Vec2;
use rand::{thread_rng, Rng};

pub struct Selection {
    pub entity: Entity,
    pub position: Option<Position>,
    pub range: Option<Range>,
    pub damage: Option<Damage>,
    pub speed: Option<Speed>,
    pub health: Option<Health>,
    pub score: Option<Score>,
}

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
        let range = Range {
            squared: rng.gen_range(10_000..20_000),
        };
        let score = Score(0);
        let target = Target {
            position: None,
            entity: None,
        };
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
    let range = Range {
        squared: rng.gen_range(10_000..20_000),
    };
    let score = Score(0);
    let target = Target {
        position: None,
        entity: None,
    };

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
        .filter(|(_id, p)| (*p - &remove_position).norm_squared() < 100i32)
        .min_by_key(|(_id, p)| (*p - &remove_position).norm_squared())
        .map(|(id, _p)| id);
    if let Some(id) = closest_entity_to_position {
        world.despawn(id).unwrap();
    }
}

pub fn get_selection(world: &mut World, entity: Entity) -> Selection {
    let damage = world.get_mut::<Damage>(entity).ok();
    let health = world.get_mut::<Health>(entity).ok();
    let range = world.get_mut::<Range>(entity).ok();
    let speed = world.get_mut::<Speed>(entity).ok();
    let position = world.get_mut::<Position>(entity).ok();
    let score = world.get_mut::<Score>(entity).ok();
    Selection {
        entity,
        position: position.map(|p| p.clone()),
        range: range.map(|r| r.clone()),
        damage: damage.map(|d| d.clone()),
        speed: speed.map(|s| s.clone()),
        health: health.map(|h| h.clone()),
        score: score.map(|s| s.clone()),
    }
}

pub fn closest_entity(world: &mut World, position: &Vec2) -> Option<Selection> {
    let target = Position {
        x: position.x as i32,
        y: position.y as i32,
    };
    let closest_entity_to_position = world
        .query::<&Position>()
        .iter()
        .filter(|(_id, p)| (*p - &target).norm_squared() < 100i32)
        .min_by_key(|(_id, p)| (*p - &target).norm_squared())
        .map(|(id, _p)| id);
    closest_entity_to_position.map(|id| get_selection(world, id))
}
