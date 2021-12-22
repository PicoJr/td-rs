#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
use clap::{App, Arg};

mod components;
mod spawns;
mod systems;

use hecs::*;
use std::io;

fn print_world_state(world: &mut World) {
    println!("\nEntity stats:");
    for (id, (health, position)) in &mut world.query::<(
        &components::components::Health,
        &components::components::Position,
    )>() {
        println!("(unit) ID: {:?}, {:?} {:?}", id, health, position);
    }
    for (id, (position, damage, range, score)) in &mut world.query::<(
        &components::components::Position,
        &components::components::Damage,
        &components::components::Range,
        &components::components::Score,
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

fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Does awesome things")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .takes_value(false)
                .required(false)
                .help("prompt between simulation steps"),
        )
        .get_matches();

    let interactive = matches.is_present("interactive");

    let target = components::components::Position { x: 0, y: 0 };
    let mut world = World::new();

    spawns::spawns::batch_spawn_units(&mut world, 500);
    spawns::spawns::batch_spawn_towers(&mut world, 10);

    let mut motion_query = PreparedQuery::<(
        &mut components::components::Position,
        &components::components::Speed,
    )>::default();

    loop {
        let action = if interactive {
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
                systems::systems::system_integrate_motion(&mut world, &mut motion_query, &target);
                let _removed = systems::systems::system_remove_arrived(&mut world, &target);
                systems::systems::system_fire_at_closest(&mut world);
                let units_left = systems::systems::system_units_left(&world);
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
}
