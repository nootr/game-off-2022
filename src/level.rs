use bevy::prelude::*;

use crate::game::{GameState, Volatile};
use crate::grid::get_coordinates;
use crate::physics::{Collider, Solid};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_walls));
    }
}

fn setup_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = 1;
    let locations = match level {
        1 => vec![
            // Upper wall
            (10, 14),
            (11, 14),
            (12, 14),
            (13, 14),
            (14, 14),
            // Left wall
            (10, 11),
            (10, 12),
            (10, 13),
            // Right wall
            (14, 11),
            (14, 12),
            (14, 13),
            // Lower wall
            (10, 10),
            (11, 10),
            (13, 10),
            (14, 10),
        ],
        _ => Vec::new(),
    };

    for (row, column) in locations {
        let coordinates = get_coordinates(row, column);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/wall.png"),
                transform: Transform {
                    translation: coordinates.extend(-1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            },
            Collider { ..default() },
            Volatile,
            Solid,
        ));
    }
}
