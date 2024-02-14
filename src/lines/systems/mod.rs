mod draw_line;
use self::draw_line::*;

mod handle_input;
use self::handle_input::*;

mod setup;
use self::setup::*;

pub fn register_systems(app: &mut bevy::prelude::App) {
    use bevy::prelude::*;

    app.add_systems(Startup, setup);
    app.add_systems(Update, (draw_line, handle_input));
}
