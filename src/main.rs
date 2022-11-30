use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use game_off_2022::GamePlugins;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Trumpet Trainee".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(AudioPlugin)
        .add_plugins(GamePlugins)
        .run();
}
