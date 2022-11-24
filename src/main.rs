use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;

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
        .add_plugin(AudioPlugin)
        .add_plugins(GamePlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
