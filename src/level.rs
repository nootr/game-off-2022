use bevy::prelude::*;

use crate::game::{GameState, Volatile};
use crate::grid::snap;
use crate::physics::{Collider, Solid};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_walls));
    }
}

fn setup_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    for y in 0..10 {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/wall.png"),
                transform: Transform {
                    translation: snap(Vec2::new(-300.0, (y as f32 - 5.0) * 40.0)).extend(-1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            })
            .insert(Collider { ..default() })
            .insert(Volatile)
            .insert(Solid);
    }
    for x in 0..10 {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/wall.png"),
                transform: Transform {
                    translation: snap(Vec2::new(-300.0 + 50.0 * (x as f32), -210.0)).extend(-1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            })
            .insert(Collider { ..default() })
            .insert(Volatile)
            .insert(Solid);
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/wall.png"),
                transform: Transform {
                    translation: snap(Vec2::new(-300.0 + 50.0 * (x as f32), 200.0)).extend(-1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            })
            .insert(Collider { ..default() })
            .insert(Volatile)
            .insert(Solid);
    }
}
