use bevy::prelude::*;

use game_off_2022::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Trumpet Trainee".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugins(GamePlugins)
        .run();
}
