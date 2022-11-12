use bevy::prelude::*;

use crate::camera::CameraShake;
use crate::game::{GameState, Volatile};
use crate::physics::{Collider, ColliderBundle, Solid};
use crate::sprite::AnimationTimer;

#[derive(Component)]
pub struct Tower;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_tower))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(hit_tower))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(remove_tower));
    }
}

fn setup_tower(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/tower.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Tower)
        .insert_bundle(ColliderBundle {
            collider: Collider {
                hit_box: Vec2::new(24.0 * 4.0, 24.0 * 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Solid)
        .insert(Volatile);
}

fn hit_tower(
    mut game_state: ResMut<State<GameState>>,
    tower_query: Query<&Collider, With<Tower>>,
    mut camera_query: Query<&mut CameraShake, Without<Tower>>,
) {
    let tower_collider = tower_query.single();

    if !tower_collider.hit {
        return;
    }

    let mut shake = camera_query.single_mut();
    shake.trauma += 0.7;

    game_state.set(GameState::GameOver).unwrap();
}

fn remove_tower(mut commands: Commands, tower_query: Query<Entity, With<Tower>>) {
    let tower = tower_query.single();

    commands.entity(tower).despawn();
}
