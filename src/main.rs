#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod components;
mod config;
mod spawns;
mod systems;

use crate::config::get_config;
use hecs::*;
use std::io;

fn print_world_state(world: &mut World) {
    println!("\nEntity stats:");
    for (id, (health, position)) in
        &mut world.query::<(&components::Health, &components::Position)>()
    {
        println!("(unit) ID: {:?}, {:?} {:?}", id, health, position);
    }
    for (id, (position, damage, range, score)) in &mut world.query::<(
        &components::Position,
        &components::Damage,
        &components::Range,
        &components::Score,
    )>() {
        println!(
            "(tower) ID: {:?}, {:?} {:?} {:?} {:?}",
            id, position, damage, range, score
        );
    }
}

enum Action {
    Quit,
    Print,
    Continue,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = get_config()?;

    let target = components::Position { x: 0, y: 0 };
    let mut world = World::new();

    spawns::batch_spawn_units(&mut world, config.units);
    spawns::batch_spawn_towers(&mut world, config.towers);

    let mut motion_query =
        PreparedQuery::<(&mut components::Position, &components::Speed)>::default();

    loop {
        let action = if config.interactive {
            println!("\n'Enter' to continue simulation, '?' for entity list, 'q' to quit");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "" => Action::Continue,
                "q" => Action::Quit,
                "?" => Action::Print,
                _ => Action::Continue,
            }
        } else {
            Action::Continue
        };
        match action {
            Action::Continue => {
                // Run all simulation systems:
                systems::system_integrate_motion(&mut world, &mut motion_query, &target);
                let _removed = systems::system_remove_arrived(&mut world, &target);
                systems::system_fire_at_closest(&mut world);
                let units_left = systems::system_units_left(&world);
                if units_left == 0 {
                    break;
                }
            }
            Action::Print => {
                print_world_state(&mut world);
            }
            Action::Quit => {
                break;
            }
        }
    }
    let score = systems::system_score(&world);
    info!("score: {}", score);
    Ok(())
}
