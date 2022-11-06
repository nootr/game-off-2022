use bevy::prelude::*;

use crate::physics::{Collider, Solid};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_walls);
    }
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/wall.png"),
            transform: Transform {
                translation: Vec3::new(-300.0, 0.0, 0.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            ..default()
        })
        .insert(Collider {
            ..Default::default()
        })
        .insert(Solid);
}
