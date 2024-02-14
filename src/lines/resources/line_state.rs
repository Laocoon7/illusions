use bevy::prelude::*;

#[derive(Resource)]
pub struct LineState {
    pub timer: Timer,
    pub color_a: Color,
    pub color_b: Color,
    pub random_colors: bool,
}

impl Default for LineState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            color_a: Color::BLACK,
            color_b: Color::WHITE,
            random_colors: false,
        }
    }
}
