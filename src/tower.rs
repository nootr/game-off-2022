use bevy::prelude::*;

use crate::camera::CameraShake;
use crate::game::{GameState, Volatile};
use crate::physics::{Collider, ColliderBundle, Solid};
use crate::sprite::AnimationTimer;

#[allow(dead_code)] // The unused handles might come in handy later
#[derive(Component, Default)]
pub struct Tower {
    idle_handle: Handle<TextureAtlas>,
    scared_handle: Handle<TextureAtlas>,
    side_handle: Handle<TextureAtlas>,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_tower))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(hit_tower));
    }
}

fn setup_tower(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_texture_handle = asset_server.load("sprites/spritesheet_elephant_front_idle.png");
    let idle_texture_atlas =
        TextureAtlas::from_grid(idle_texture_handle, Vec2::new(32.0, 32.0), 6, 1, None, None);
    let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);

    let scared_texture_handle = asset_server.load("sprites/spritesheet_elephant_panicked.png");
    let scared_texture_atlas = TextureAtlas::from_grid(
        scared_texture_handle,
        Vec2::new(32.0, 32.0),
        6,
        1,
        None,
        None,
    );
    let scared_texture_atlas_handle = texture_atlases.add(scared_texture_atlas);

    let side_texture_handle = asset_server.load("sprites/spritesheet_elephant_side_idle.png");
    let side_texture_atlas =
        TextureAtlas::from_grid(side_texture_handle, Vec2::new(32.0, 32.0), 6, 1, None, None);
    let side_texture_atlas_handle = texture_atlases.add(side_texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: idle_texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(4.0))
                .with_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Tower {
            idle_handle: idle_texture_atlas_handle,
            scared_handle: scared_texture_atlas_handle,
            side_handle: side_texture_atlas_handle,
        },
        ColliderBundle {
            collider: Collider {
                hit_box: Vec2::new(30.0 * 4.0, 30.0 * 4.0),
                ..default()
            },
            ..default()
        },
        Solid,
        Volatile,
    ));
}

fn hit_tower(
    mut game_state: ResMut<State<GameState>>,
    mut tower_query: Query<(&Tower, &mut Handle<TextureAtlas>, &Collider)>,
    mut camera_query: Query<&mut CameraShake, Without<Tower>>,
) {
    let (tower, mut atlas, tower_collider) = tower_query.single_mut();

    if !tower_collider.hit {
        return;
    }

    let mut shake = camera_query.single_mut();
    shake.trauma += 0.7;

    game_state.set(GameState::GameOver).unwrap();

    *atlas = tower.scared_handle.clone();
}
