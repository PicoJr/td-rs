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
use macroquad::prelude::{
    clear_background, draw_text, next_frame, screen_height, screen_width, DARKGRAY, GREEN, RED,
    WHITE,
};
use macroquad::shapes::draw_circle;
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

fn draw_world(world: &World) {
    let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);
    // let (center_x, center_y) = (100f32, 100f32);
    for (_id, position) in world
        .query::<With<components::Health, &components::Position>>()
        .iter()
    {
        draw_circle(
            center_x + position.x as f32,
            center_y + position.y as f32,
            10.0,
            RED,
        );
    }
    for (_id, position) in world
        .query::<With<components::Damage, &components::Position>>()
        .iter()
    {
        draw_circle(
            center_x + position.x as f32,
            center_y + position.y as f32,
            10.0,
            GREEN,
        );
    }
}

enum Action {
    Quit,
    Print,
    Continue,
}

#[macroquad::main("TD")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = get_config()?;

    let target = components::Position { x: 0, y: 0 };
    let mut world = World::new();

    spawns::batch_spawn_units(&mut world, config.units);
    spawns::batch_spawn_towers(&mut world, config.towers);

    let mut motion_query =
        PreparedQuery::<(&mut components::Position, &components::Speed)>::default();
    let mut step: usize = 0;

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
        clear_background(WHITE);
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
        draw_world(&world);
        draw_text(&format!("step: {}", step), 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await;
        step += 1;
    }
    let score = systems::system_score(&world);
    info!("score: {}", score);
    Ok(())
}
