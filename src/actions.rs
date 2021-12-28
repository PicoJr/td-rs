use macroquad::prelude::{
    is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position, Camera2D, KeyCode,
    MouseButton, Vec2,
};

pub(crate) enum Action {
    Build(Vec2),
    ChangeMode(Mode),
    View(Vec2),
    Remove(Vec2),
    PrintState,
    Quit,
    Spawn,
    TogglePause,
    ToggleDebug,
}

pub(crate) enum CameraAction {
    Zoom(f32),
    Target(f32, f32),
}

#[derive(Debug)]
pub(crate) enum Mode {
    Build,
    Remove,
    View,
}

pub(crate) fn read_camera_action() -> Option<CameraAction> {
    if is_key_down(KeyCode::Left) {
        Some(CameraAction::Target(0.1, 0.0))
    } else if is_key_down(KeyCode::Right) {
        Some(CameraAction::Target(-0.1, 0.0))
    } else if is_key_down(KeyCode::Up) {
        Some(CameraAction::Target(0.0, -0.1))
    } else if is_key_down(KeyCode::Down) {
        Some(CameraAction::Target(0.0, 0.1))
    } else if is_key_down(KeyCode::J) {
        Some(CameraAction::Zoom(0.9))
    } else if is_key_down(KeyCode::K) {
        Some(CameraAction::Zoom(1.1))
    } else {
        None
    }
}

pub(crate) fn read_simulation_action(camera: &Camera2D, mode: &Mode) -> Option<Action> {
    if is_key_pressed(KeyCode::Space) {
        Some(Action::TogglePause)
    } else if is_key_pressed(KeyCode::D) {
        Some(Action::ChangeMode(Mode::Remove))
    } else if is_key_pressed(KeyCode::I) {
        Some(Action::ChangeMode(Mode::Build))
    } else if is_key_pressed(KeyCode::P) {
        Some(Action::PrintState)
    } else if is_key_pressed(KeyCode::Q) {
        Some(Action::Quit)
    } else if is_key_pressed(KeyCode::R) {
        Some(Action::Spawn)
    } else if is_key_pressed(KeyCode::V) {
        Some(Action::ChangeMode(Mode::View))
    } else if is_key_pressed(KeyCode::Equal) {
        Some(Action::ToggleDebug)
    } else if is_mouse_button_pressed(MouseButton::Left) {
        let world_position = camera.screen_to_world(Vec2::from(mouse_position()));
        match mode {
            Mode::Build => Some(Action::Build(world_position)),
            Mode::Remove => Some(Action::Remove(world_position)),
            Mode::View => Some(Action::View(world_position)),
        }
    } else {
        None
    }
}
