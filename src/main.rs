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
use macroquad::input::is_key_pressed;
use macroquad::prelude::{
    clear_background, draw_text, is_key_down, next_frame, screen_height, screen_width, set_camera,
    set_default_camera, vec2, Camera2D, KeyCode, DARKGRAY, GREEN, RED, WHITE,
};
use macroquad::shapes::draw_circle;

const TOWER_RADIUS: f32 = 10.0;
const UNIT_RADIUS: f32 = 5.0;

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
    let (center_x, center_y) = (0., 0.);
    for (_id, position) in world
        .query::<With<components::Health, &components::Position>>()
        .iter()
    {
        draw_circle(
            center_x + position.x as f32,
            center_y + position.y as f32,
            UNIT_RADIUS,
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
            TOWER_RADIUS,
            GREEN,
        );
    }
}

enum Action {
    Quit,
    TogglePause,
    Spawn,
}

enum CameraAction {
    Zoom(f32),
    Target(f32, f32),
}

fn read_camera_action() -> Option<CameraAction> {
    if is_key_down(KeyCode::Left) {
        Some(CameraAction::Target(-0.1, 0.0))
    } else if is_key_down(KeyCode::Right) {
        Some(CameraAction::Target(0.1, 0.0))
    } else if is_key_down(KeyCode::Up) {
        Some(CameraAction::Target(0.0, 0.1))
    } else if is_key_down(KeyCode::Down) {
        Some(CameraAction::Target(0.0, -0.1))
    } else if is_key_down(KeyCode::J) {
        Some(CameraAction::Zoom(0.9))
    } else if is_key_down(KeyCode::K) {
        Some(CameraAction::Zoom(1.1))
    } else {
        None
    }
}

fn read_simulation_action() -> Option<Action> {
    if is_key_pressed(KeyCode::Space) {
        Some(Action::TogglePause)
    } else if is_key_pressed(KeyCode::R) {
        Some(Action::Spawn)
    } else if is_key_pressed(KeyCode::Q) {
        Some(Action::Quit)
    } else {
        None
    }
}

#[macroquad::main("TD")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = get_config()?;

    let target = components::Position { x: 0, y: 0 };
    let mut world = World::new();
    let mut zoom = 0.001;
    let mut camera_target = (0., 0.);
    let mut pause: bool = config.paused;

    spawns::batch_spawn_units(&mut world, config.units);
    spawns::batch_spawn_towers(&mut world, config.towers);

    let mut motion_query =
        PreparedQuery::<(&mut components::Position, &components::Speed)>::default();
    let mut step: usize = 0;

    loop {
        match read_camera_action() {
            None => {}
            Some(CameraAction::Zoom(z)) => {
                zoom *= z;
            }
            Some(CameraAction::Target(t0, t1)) => {
                camera_target.0 += t0;
                camera_target.1 += t1;
            }
        }
        match read_simulation_action() {
            Some(Action::Quit) => {
                break;
            }
            Some(Action::TogglePause) => {
                pause = !pause;
                print_world_state(&mut world);
            }
            Some(Action::Spawn) => {
                spawns::batch_spawn_units(&mut world, config.units);
            }
            None => {}
        };

        if !pause {
            systems::system_integrate_motion(&mut world, &mut motion_query, &target);
            let _removed = systems::system_remove_arrived(&mut world, &target);
            systems::system_fire_at_closest(&mut world);
            let _units_left = systems::system_units_left(&world);
            step += 1;
        }

        clear_background(WHITE);

        set_camera(&Camera2D {
            target: vec2(camera_target.0, camera_target.1),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            offset: vec2(camera_target.0, camera_target.1),
            ..Default::default()
        });
        draw_world(&world);

        set_default_camera();
        draw_text(
            &format!(
                "step: {}, zoom: {}, target: {:?}",
                step, zoom, camera_target
            ),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );
        next_frame().await;
    }
    let score = systems::system_score(&world);
    info!("score: {}", score);
    Ok(())
}
