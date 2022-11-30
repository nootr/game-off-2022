use bevy::prelude::*;
use rand::Rng;

use crate::game::{GameState, Volatile};
use crate::grid::get_coordinates;
use crate::physics::{Collider, Solid};

#[derive(Debug, Default, Resource)]
pub struct Level {
    pub level: u8,
}

impl Level {
    pub fn title(&self) -> String {
        match self.level {
            1 => "1st floor: IT department".to_string(),
            2 => "2nd floor: Sales department".to_string(),
            3 => "3rd floor: Conference room".to_string(),
            4 => "4th floor: Executive room".to_string(),
            _ => "???".to_string(),
        }
    }

    pub fn help_text(&self) -> Option<String> {
        match self.level {
            1 => Some("Place boxes to block co-workers".to_string()),
            2 => Some("Coffee attracts co-workers".to_string()),
            3 => Some("People try to avoid the manager".to_string()),
            _ => None,
        }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level { level: 1 })
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_walls))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_floor));
    }
}

pub fn setup_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            (None, 12, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 13, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 14, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 15, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 16, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 17, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 18, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 19, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 12, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 13, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 14, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 15, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 16, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 17, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 18, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 19, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 11, 15, "sprites/plant_A.png", 1.0, false),
            // Desk
            (
                None,
                6,
                14,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                7,
                14,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (
                None,
                8,
                14,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 6, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 6, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 7, 13, "sprites/desk_B.png", 1.5, false),
            (None, 7, 12, "sprites/office_chair_back.png", 2.0, false),
            // Desk
            (
                None,
                10,
                14,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                11,
                14,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (
                None,
                12,
                14,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 10, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 10, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 11, 13, "sprites/desk_D.png", 1.5, false),
            // Desk
            (
                None,
                10,
                11,
                "sprites/Cubicle_screen_corner_B.png",
                1.0,
                true,
            ),
            (
                None,
                11,
                11,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (
                None,
                12,
                11,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 10, 10, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 10, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 11, 10, "sprites/desk_C.png", 1.5, false),
            // Desk
            (
                None,
                14,
                14,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                15,
                14,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (
                None,
                16,
                14,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 14, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 14, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 15, 13, "sprites/desk_D.png", 1.5, false),
            // Desk
            (
                None,
                14,
                11,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                15,
                11,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                true,
            ),
            (
                None,
                16,
                11,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (None, 14, 10, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 14, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 15, 10, "sprites/desk_B.png", 1.5, false),
            // Objects
            (None, 16, 12, "sprites/plant_B.png", 1.0, false),
            (None, 11, 12, "sprites/BoxB.png", 1.5, false),
        ],
        2 => vec![
            // Upper wall
            (None, 4, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 5, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 6, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 7, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 8, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 9, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 10, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 11, 16, "sprites/archive_cabinet.png", 1.1, false),
            (None, 4, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 5, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 6, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 7, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 8, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 9, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 10, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 11, 15, "sprites/archive_cabinet.png", 1.1, false),
            (None, 12, 15, "sprites/plant_A.png", 1.0, false),
            // Desk
            (
                None,
                6,
                14,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                7,
                14,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (
                None,
                8,
                14,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 6, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 6, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 7, 13, "sprites/desk_B.png", 1.5, false),
            (None, 7, 12, "sprites/office_chair_back.png", 2.0, false),
            // Desk
            (
                None,
                10,
                14,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                11,
                14,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (
                None,
                12,
                14,
                "sprites/Cubicle_screen_square_A.png",
                1.0,
                false,
            ),
            (None, 10, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 10, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 11, 13, "sprites/desk_D.png", 1.5, false),
            // Desk
            (
                None,
                14,
                11,
                "sprites/Cubicle_screen_corner_A.png",
                1.0,
                true,
            ),
            (
                None,
                15,
                11,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                true,
            ),
            (
                None,
                16,
                11,
                "sprites/Cubicle_screen_square_B.png",
                1.0,
                false,
            ),
            (None, 14, 10, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 14, 11, "sprites/Cubicle_screen_side.png", 1.0, false),
            (None, 15, 10, "sprites/desk_B.png", 1.5, false),
            // Objects
            (None, 17, 15, "sprites/plant_B.png", 1.0, false),
            (None, 18, 14, "sprites/plant_A.png", 1.0, true),
        ],
        3 => vec![
            (None, 11, 15, "sprites/office_chair.png", 2.0, false),
            (None, 12, 15, "sprites/office_chair.png", 2.0, false),
            (None, 13, 15, "sprites/office_chair.png", 2.0, false),
            (None, 14, 15, "sprites/office_chair.png", 2.0, false),
            (None, 15, 15, "sprites/office_chair.png", 2.0, false),
            (
                Some(Vec2::new(5.0 * 4.0 * 24.0, 4.0 * 18.0)),
                13,
                14,
                "sprites/conference_desk.png",
                2.0,
                true,
            ),
            (
                Some(Vec2::new(5.0 * 4.0 * 24.0, 4.0 * 18.0)),
                13,
                11,
                "sprites/conference_desk.png",
                2.0,
                true,
            ),
            (None, 10, 11, "sprites/plant_B.png", 1.0, false),
            (None, 10, 14, "sprites/plant_A.png", 1.0, true),
            (None, 11, 10, "sprites/office_chair_back.png", 2.0, false),
            (None, 12, 10, "sprites/office_chair_back.png", 2.0, false),
            (None, 13, 10, "sprites/office_chair_back.png", 2.0, false),
            (None, 14, 10, "sprites/office_chair_back.png", 2.0, false),
            (None, 15, 10, "sprites/office_chair_back.png", 2.0, false),
        ],
        4 => vec![
            (None, 13, 15, "sprites/office_chair.png", 2.0, false),
            (
                Some(Vec2::new(5.0 * 4.0 * 24.0, 4.0 * 18.0)),
                13,
                14,
                "sprites/conference_desk.png",
                2.0,
                true,
            ),
            (None, 10, 14, "sprites/plant_A.png", 1.0, false),
            (None, 16, 14, "sprites/plant_B.png", 1.0, true),
        ],
        _ => Vec::new(),
    };

    let mut z = -1.0;
    for (some_hit_box, row, column, sprite, extra_scale, flipped) in walls {
        let collider = match some_hit_box {
            Some(hit_box) => Collider {
                hit_box,
                ..default()
            },
            _ => Collider::default(),
        };

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
                },
                ..default()
            },
            collider,
            Volatile,
            Solid,
        ));
        z += 0.01;
    }
}
