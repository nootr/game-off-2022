use bevy::prelude::*;

use crate::game::{Game, GameState};
use crate::physics::{Collider, ColliderBundle, Solid};
use crate::sprite::AnimationTimer;

#[derive(Component)]
pub struct Tower;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_tower).add_system(hit_system);
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
        .insert(Solid);
}

fn hit_system(mut game: ResMut<Game>, mut tower_query: Query<&Collider, With<Tower>>) {
    let tower_collider = tower_query.single_mut();

    if tower_collider.hit && game.state == GameState::Running {
        game.state = GameState::GameOver;
    }
}
