use bevy::prelude::*;

use bevy_new_minimal_crate::NewPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NewPlugin)
        .run();
}
