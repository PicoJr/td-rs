#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod actions;
mod components;
mod config;
mod spawns;
mod systems;

use crate::actions::{read_camera_action, read_simulation_action, Action, CameraAction, Mode};
use crate::config::get_config;
use crate::spawns::Selection;
use hecs::*;
use macroquad::prelude::{
    clear_background, draw_line, draw_rectangle_lines, draw_text, get_fps, next_frame,
    screen_height, screen_width, set_camera, set_default_camera, vec2, Camera2D, Color, Vec2,
    BLACK, DARKGRAY, GREEN, RED, WHITE,
};
use macroquad::shapes::{draw_circle, draw_rectangle};

const TOWER_RADIUS: f32 = 10.0;
const UNIT_RADIUS: f32 = 5.0;
const LASER_WIDTH: f32 = 2.0;
const WAYPOINTS_WIDTH: f32 = 2.0;
const RANGE_WIDTH: f32 = 2.0;

fn print_world_state(world: &mut World) {
    println!("\nEntity stats:");
    for (id, (health, position)) in
        &mut world.query::<(&components::Health, &components::Position)>()
    {
        println!("(unit) ID: {:?}, {:?} {:?}", id, health, position);
    }
    for (id, (position, damage, range, score, target)) in &mut world.query::<(
        &components::Position,
        &components::Damage,
        &components::Range,
        &components::Score,
        &components::Target,
    )>() {
        println!(
            "(tower) ID: {:?}, {:?} {:?} {:?} {:?} {:?}",
            id, position, damage, range, score, target,
        );
    }
}

fn draw_world(world: &World) {
    for (_id, (health, position)) in world
        .query::<(&components::Health, &components::Position)>()
        .iter()
    {
        let health_ratio = (health.value as f32 / health.max as f32).clamp(0f32, 1f32);
        let color = Color::new(health_ratio, 0.0, 1.0 - health_ratio, 1.0f32);
        draw_circle(position.x as f32, position.y as f32, UNIT_RADIUS, color);
    }
    for (_id, position) in world
        .query::<With<components::Damage, &components::Position>>()
        .iter()
    {
        draw_rectangle(
            position.x as f32 - TOWER_RADIUS * 0.5,
            position.y as f32 - TOWER_RADIUS * 0.5,
            TOWER_RADIUS,
            TOWER_RADIUS,
            GREEN,
        );
    }
    for (_id, (target, position)) in world
        .query::<(&components::Target, &components::Position)>()
        .iter()
    {
        if let Some(target_position) = &target.position {
            draw_line(
                target_position.x as f32,
                target_position.y as f32,
                position.x as f32,
                position.y as f32,
                LASER_WIDTH,
                RED,
            );
        }
    }
}

fn draw_waypoints(waypoints: &[components::Position]) {
    for (p0, p1) in waypoints.iter().zip(waypoints.iter().skip(1)) {
        draw_line(
            p0.x as f32,
            p0.y as f32,
            p1.x as f32,
            p1.y as f32,
            WAYPOINTS_WIDTH,
            BLACK,
        );
    }
}

#[macroquad::main("TD")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = get_config()?;

    let waypoints = vec![
        components::Position { x: -1000, y: 1000 },
        components::Position { x: 0, y: 0 },
    ];
    let start = waypoints.first().expect("waypoints not empty");
    let end = waypoints.last().expect("waypoints not empty");

    let mut world = World::new();
    let mut zoom = 0.001;
    let mut camera_target = (0., 0.);
    let mut pause: bool = config.paused;
    let mut debug: bool = false;
    let mut camera: Camera2D;
    let mut mode: Mode = Mode::View;

    spawns::batch_spawn_units(&mut world, config.units, start);
    spawns::batch_spawn_towers(&mut world, config.towers);

    let mut motion_query = PreparedQuery::<(
        &mut components::Position,
        &mut components::Waypoint,
        &components::Speed,
    )>::default();
    let mut step: usize = 0;
    let mut arrived: usize = 0;
    let mut selection: Option<Selection> = None;

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

        camera = Camera2D {
            target: vec2(camera_target.0, camera_target.1),
            zoom: vec2(zoom, -zoom * screen_width() / screen_height()),
            offset: vec2(camera_target.0, camera_target.1),
            ..Default::default()
        };

        match read_simulation_action(&camera, &mode) {
            Some(Action::Quit) => {
                break;
            }
            Some(Action::TogglePause) => {
                pause = !pause;
            }
            Some(Action::Spawn) => {
                spawns::batch_spawn_units(&mut world, config.units, start);
            }
            Some(Action::ToggleDebug) => {
                debug = !debug;
            }
            Some(Action::PrintState) => {
                print_world_state(&mut world);
            }
            Some(Action::Build(build_position)) => {
                spawns::spawn_tower(&mut world, &build_position);
            }
            Some(Action::View(view_position)) => {
                selection = spawns::closest_entity(&mut world, &view_position);
            }
            Some(Action::Remove(remove_position)) => {
                spawns::remove_tower(&mut world, &remove_position);
            }
            Some(Action::ChangeMode(new_mode)) => {
                mode = new_mode;
            }
            None => {}
        };

        if !pause {
            systems::system_integrate_motion(&mut world, &mut motion_query, waypoints.as_slice());
            systems::system_remove_dead(&mut world);
            let removed = systems::system_remove_arrived(&mut world, end);
            arrived += removed;
            systems::system_fire_at_closest(&mut world);
            step += 1;
        }

        clear_background(WHITE);

        set_camera(&camera);
        draw_world(&world);
        if debug {
            draw_waypoints(waypoints.as_slice());
        }
        match &selection {
            None => {}
            Some(selection) => {
                if let Some(range) = &selection.range {
                    draw_rectangle_lines(
                        selection.position.x as f32 - range.0 as f32 * 0.5,
                        selection.position.y as f32 - range.0 as f32 * 0.5,
                        range.0 as f32,
                        range.0 as f32,
                        RANGE_WIDTH,
                        BLACK,
                    );
                }
                if let Some(damage) = &selection.damage {
                    draw_text(
                        &format!("damage: {:?}", damage),
                        selection.position.x as f32,
                        selection.position.y as f32,
                        20.0,
                        BLACK,
                    );
                }
            }
        }

        set_default_camera();
        if debug {
            let units = systems::system_units_left(&world);
            draw_text(
                &format!("units: {}, arrived: {}, mode: {:?}", units, arrived, mode),
                20.0,
                20.0,
                30.0,
                DARKGRAY,
            );
            draw_text(
                &format!(
                    "fps: {} step: {} zoom: {} camera: {:?}",
                    get_fps(),
                    step,
                    zoom,
                    camera_target
                ),
                20.0,
                40.0,
                30.0,
                DARKGRAY,
            );
        }
        next_frame().await;
    }
    let score = systems::system_score(&world);
    info!("score: {}", score);
    Ok(())
}
