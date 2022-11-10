use bevy::prelude::*;

use game_off_2022::GamePlugins;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Trumpet Trainee".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
