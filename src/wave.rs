use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::enemies::Enemy;
use crate::force::{Force, ForceType};
use crate::game::{GameState, Volatile};
use crate::physics::{Collider, ColliderBundle, Moving};
use crate::sprite::AnimationTimer;

struct EnemySpawnEvent {
    influence: f32,
    force_type: ForceType,
}

#[derive(Clone, Copy, Debug)]
pub struct EnemySpawn {
    pub spawn_time: f32,
    influence: f32,
    force_type: ForceType,
}

impl Default for EnemySpawn {
    fn default() -> Self {
        EnemySpawn {
            spawn_time: 0.0,
            influence: 80.0,
            force_type: ForceType::Passive,
        }
    }
}

#[derive(Debug, Default, Resource)]
struct EnemySpawnQueue {
    pub enemies: Vec<EnemySpawn>,
}

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnQueue::default())
            .add_event::<EnemySpawnEvent>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_wave))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(tick_wave))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup_wave));
    }
}

fn setup_wave(mut enemy_queue: ResMut<EnemySpawnQueue>) {
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 4.0,
        ..default()
    });
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 5.0,
        ..default()
    });
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 9.0,
        force_type: ForceType::Repel,
        ..default()
    });
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 13.0,
        ..default()
    });
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 16.0,
        force_type: ForceType::Attract,
        ..default()
    });
    enemy_queue.enemies.push(EnemySpawn {
        spawn_time: 25.0,
        ..default()
    });
}

fn tick_wave(
    mut game_state: ResMut<State<GameState>>,
    mut enemy_queue: ResMut<EnemySpawnQueue>,
    mut ev_spawn_enemy: EventWriter<EnemySpawnEvent>,
    time: Res<Time>,
) {
    if enemy_queue.enemies.is_empty() {
        game_state.set(GameState::Won).unwrap();
    }

    let time_delta = time.delta_seconds();

    enemy_queue.enemies.retain_mut(|enemy_spawn| {
        enemy_spawn.spawn_time -= time_delta;
        if enemy_spawn.spawn_time < 0.0 {
            ev_spawn_enemy.send(EnemySpawnEvent {
                influence: enemy_spawn.influence,
                force_type: enemy_spawn.force_type,
            });

            false
        } else {
            true
        }
    });
}

fn spawn_enemy(
    mut commands: Commands,
    mut ev_spawn_enemy: EventReader<EnemySpawnEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in ev_spawn_enemy.iter() {
        let window = windows.primary();
        let half_width = window.width() as f32 * 0.5;
        let half_height = window.height() as f32 * 0.5;

        let texture_handle = asset_server.load("sprites/enemy.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let height: f32 = rand::thread_rng().gen_range(-half_height..half_height);

        let moving = Moving::new(Vec3::new(300.0, 0.0, 0.0));

        let mut color = Color::from(ev.force_type);
        color.set_a(match ev.force_type {
            ForceType::Passive => 0.0,
            _ => 0.5,
        });

        let force_field = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle::new(ev.influence)))
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            })
            .insert(Volatile)
            .id();

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(-half_width - 4.0 * 12.0, height, 0.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            })
            .insert(ColliderBundle {
                collider: Collider {
                    hit_box: Vec2::new(24.0 * 4.0, 24.0 * 4.0),
                    ..default()
                },
                moving,
            })
            .insert(AnimationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .insert(Enemy)
            .insert(Volatile)
            .insert(Force {
                newton: 500.0,
                influence: ev.influence,
                force_type: ev.force_type,
            })
            .push_children(&[force_field]);
    }
}

fn cleanup_wave(mut enemy_queue: ResMut<EnemySpawnQueue>) {
    enemy_queue.enemies.clear();
}
