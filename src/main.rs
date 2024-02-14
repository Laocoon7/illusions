use bevy::prelude::*;

use crate::lines::LinesPlugin;

pub mod lines;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins(LinesPlugin);

    app.run();
}
