use crate::components::{Damage, Distance, Health, Position, Range, Score, Speed};

fn direction(movement: Distance) -> Distance {
    movement.clamp(-1, 1)
}

fn manhattan_distance(pos: &Position, other_pos: &Position) -> Distance {
    (pos.x - other_pos.x).abs() + (pos.y - other_pos.y).abs()
}

use hecs::{Entity, PreparedQuery, With, World};

pub fn system_integrate_motion(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Position, &Speed)>,
    target: &Position,
) {
    for (_id, (pos, spd)) in query.query_mut(world) {
        let dx: i32 = target.x - pos.x;
        let dy: i32 = target.y - pos.y;
        let dx = direction(dx) * dx.abs().min(spd.0);
        let dy = direction(dy) * dy.abs().min(spd.0);
        pos.x += dx;
        pos.y += dy;
    }
}

pub fn system_remove_arrived(world: &mut World, target: &Position) -> usize {
    let mut to_remove: Vec<Entity> = Vec::new();
    for (id, pos) in &mut world.query::<With<Health, &Position>>() {
        if pos == target {
            debug!("ID: {:?} has reached its target.", id);
            to_remove.push(id);
        }
    }

    let removed = to_remove.len();
    for entity in to_remove {
        world.despawn(entity).unwrap();
    }
    removed
}

pub fn system_remove_dead(world: &mut World) -> usize {
    let mut to_remove: Vec<Entity> = Vec::new();
    for (id, health) in &mut world.query::<&Health>() {
        if health.value <= 0 {
            debug!("ID: {:?} health is <= 0", id);
            to_remove.push(id);
        }
    }

    let removed = to_remove.len();
    for entity in to_remove {
        world.despawn(entity).unwrap();
    }
    removed
}

pub fn system_score(world: &World) -> usize {
    world
        .query::<&Score>()
        .iter()
        .map(|(_id, score)| score.0)
        .sum::<i32>() as usize
}

pub fn system_units_left(world: &World) -> usize {
    world.query::<&Health>().iter().count()
}

// In this system entities find the closest entity and fire at them
pub fn system_fire_at_closest(world: &mut World) {
    for (tower_id, (tower_position, tower_damage, tower_range, tower_score)) in
        &mut world.query::<With<Damage, (&Position, &Damage, &Range, &mut Score)>>()
    {
        let closest = world
            .query::<With<Health, &Position>>()
            .iter()
            .filter(|(target_id, target_position)| {
                *target_id != tower_id
                    && manhattan_distance(target_position, tower_position) < tower_range.0
            })
            .min_by_key(|(_, target_position)| manhattan_distance(tower_position, target_position))
            .map(|(entity, _pos)| entity);

        if let Some(entity) = closest {
            let mut target_health = world.get_mut::<Health>(entity).unwrap();

            // Is target unit still alive?
            if target_health.value > 0 {
                // apply damage
                target_health.value -= tower_damage.0;
                debug!(
                    "Unit {:?} was damaged by {:?} for {:?} HP",
                    closest, tower_id, tower_damage.0
                );
                if target_health.value <= 0 {
                    // if this killed it, increase tower score
                    tower_score.0 += 1;
                    debug!("Unit {:?} was killed by tower {:?}!", entity, tower_id);
                }
            }
        }
    }
}
