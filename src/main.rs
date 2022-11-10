use bevy::prelude::*;

use game_off_2022::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
