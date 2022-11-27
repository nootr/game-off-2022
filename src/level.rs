use bevy::prelude::*;
use rand::Rng;

use crate::game::{GameState, Volatile};
use crate::grid::get_coordinates;
use crate::physics::{Collider, Solid};

#[derive(Debug, Default, Resource)]
pub struct Level {
    pub level: u8,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level { level: 1 })
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_walls))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_floor));
    }
}

fn setup_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for row in 0..24 {
        for column in 0..24 {
            let render_floor_a: bool = rng.gen();
            let coordinates = get_coordinates(row, column);
            let sprite = match render_floor_a {
                true => "sprites/floor_texture_03A.png",
                false => "sprites/floor_texture_03B.png",
            };

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(sprite),
                    transform: Transform {
                        translation: coordinates.extend(-2.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                },
                Volatile,
            ));
        }
    }
}

fn setup_walls(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    let walls = match level.level {
        1 => vec![
            // Upper wall
            (10, 14, "sprites/Cubicle_screen_corner_A.png", 1.0, true),
            (11, 14, "sprites/Cubicle_screen_square_A.png", 1.0, false),
            (12, 14, "sprites/Cubicle_screen_square_B.png", 1.0, false),
            (13, 14, "sprites/Cubicle_screen_square_B.png", 1.0, false),
            (14, 14, "sprites/Cubicle_screen_corner_A.png", 1.0, false),
            // Left wall
            (10, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 12, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            // Right wall
            (14, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 12, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            // Lower wall
            (10, 10, "sprites/Cabinet.png", 1.5, false),
            (11, 10, "sprites/BoxA.png", 1.5, false),
            (13, 10, "sprites/BoxB.png", 1.5, false),
            (14, 10, "sprites/Cabinet.png", 1.5, false),
        ],
        2 => vec![
            // Upper wall
            (10, 14, "sprites/Cubicle_screen_corner_A.png", 1.0, true),
            (11, 14, "sprites/Cubicle_screen_square_B.png", 1.0, false),
            (12, 14, "sprites/Cubicle_screen_square_B.png", 1.0, false),
            (13, 14, "sprites/Cubicle_screen_square_A.png", 1.0, false),
            (14, 14, "sprites/Cubicle_screen_corner_B.png", 1.0, false),
            // Left wall
            (10, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 12, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (10, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            // Right wall
            (14, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 12, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (14, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
        ],
        _ => Vec::new(),
    };

    let mut z = -1.0;
    for (row, column, sprite, extra_scale, flipped) in walls {
        let coordinates = get_coordinates(row, column);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(sprite),
                transform: Transform {
                    translation: coordinates.extend(z),
                    rotation: match flipped {
                        true => Quat::from_rotation_y(std::f32::consts::PI),
                        false => Quat::default(),
                    },
                    scale: Vec3::splat(4.0 * extra_scale),
                    ..default()
                },
                ..default()
            },
            Collider { ..default() },
            Volatile,
            Solid,
        ));
        z += 0.01;
    }
}
