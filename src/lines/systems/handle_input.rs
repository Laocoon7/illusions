use std::time::Duration;

use bevy::{app::AppExit, prelude::*};

use crate::lines::resources::LineState;

pub fn handle_input(
    input: Res<Input<KeyCode>>,
    mut line_state: ResMut<LineState>,
    mut app_exit: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Space) {
        if line_state.timer.paused() {
            line_state.timer.unpause();
        } else {
            line_state.timer.pause();
        }
    } else if input.just_pressed(KeyCode::NumpadAdd) {
        let duration = line_state.timer.duration();
        if let Some(duration) = duration.checked_sub(Duration::from_secs_f32(0.1)) {
            line_state.timer.set_duration(duration);
        }
    } else if input.just_pressed(KeyCode::NumpadSubtract) {
        let duration = line_state.timer.duration();
        if let Some(duration) = duration.checked_add(Duration::from_secs_f32(0.1)) {
            line_state.timer.set_duration(duration);
        }
    } else if input.just_pressed(KeyCode::R) {
        line_state.random_colors = !line_state.random_colors;
    } else if input.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit);
    }
}
