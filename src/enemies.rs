use bevy::{prelude::*, time::FixedTimestep};
use rand::Rng;

use crate::physics::{Collider, ColliderBundle, Moving};
use crate::sprite::AnimationTimer;
use crate::tower::Tower;

#[derive(Component)]
struct Enemy;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_enemy),
        )
        .add_system(turn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    let window = windows.primary();
    let half_width = window.width() as f32 * 0.5;
    let half_height = window.height() as f32 * 0.5;

    let texture_handle = asset_server.load("sprites/enemy.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let height: f32 = rand::thread_rng().gen_range(-half_height..half_height);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(-half_width - 4.0 * 12.0, height, 0.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            ..default()
        })
        .insert_bundle(ColliderBundle {
            collider: Collider {
                hit: false,
                hit_box: Vec2::new(24.0 * 4.0, 24.0 * 4.0),
            },
            moving: Moving { velocity: 100.0 },
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Enemy);
}

fn turn_enemy(
    mut tower_query: Query<&Transform, With<Tower>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Tower>)>,
) {
    let tower_transform = tower_query.single_mut();

    for mut transform in &mut enemy_query {
        // Slowly point enemy towards tower
        let to_tower =
            (transform.translation.truncate() - tower_transform.translation.truncate()).normalize();
        transform.rotation = (transform.rotation * 99.0
            + Quat::from_rotation_arc(-Vec3::X, to_tower.extend(0.)))
            / 100.0;
    }
}