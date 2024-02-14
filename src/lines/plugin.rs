use bevy::prelude::*;

use crate::lines::systems::register_systems;

pub struct LinesPlugin;

impl Plugin for LinesPlugin {
    fn build(&self, app: &mut App) { register_systems(app); }
}
