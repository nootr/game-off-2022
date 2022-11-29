use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};
use rand::Rng;
use std::time::Duration;

use crate::enemies::Enemy;
use crate::force::{Force, ForceType};
use crate::game::{GameState, Volatile};
use crate::level::Level;
use crate::physics::{Collider, ColliderBundle, Moving};
use crate::sprite::AnimationTimer;

struct EnemySpawnEvent {
    influence: f32,
    force_type: ForceType,
    attention_span: u64,
    sprite: String,
    sprite_size: Vec2,
    location: Vec3,
}

#[derive(Debug)]
pub struct EnemySpawn {
    pub spawn_timer: Timer,
    influence: f32,
    force_type: ForceType,
    attention_span: u64,
    sprite: String,
    sprite_size: Vec2,
    location: Vec3,
}

impl Default for EnemySpawn {
    fn default() -> Self {
        let height = rand::thread_rng().gen_range(-36.0..36.0);

        EnemySpawn {
            spawn_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
            influence: 80.0,
            force_type: ForceType::Passive,
            attention_span: 15,
            sprite: "sprites/spritesheet_NPC01_M_walk.png".into(),
            sprite_size: Vec2::new(16.0, 24.0),
            location: Vec3::new(-640.0 - 4.0 * 12.0, height, 0.0),
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

fn setup_wave(level: Res<Level>, mut enemy_queue: ResMut<EnemySpawnQueue>) {
    match level.level {
        1 => {
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC01_M_walk.png".into(),
                ..default()
            });
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC02_M_walk.png".into(),
                ..default()
            });
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(20), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC02_M_walk.png".into(),
                ..default()
            });
        }
        2 => {
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC03_M_walk.png".into(),
                ..default()
            });
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC01_M_walk.png".into(),
                ..default()
            });
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(9), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC02_M_walk.png".into(),
                ..default()
            });
            enemy_queue.enemies.push(EnemySpawn {
                spawn_timer: Timer::new(Duration::from_secs(20), TimerMode::Once),
                sprite: "sprites/spritesheet_NPC03_M_walk.png".into(),
                ..default()
            });
        }
        _ => {
            // TODO: Win screen
        }
    }
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

    let time_delta = time.delta();

    enemy_queue.enemies.retain_mut(|enemy_spawn| {
        enemy_spawn.spawn_timer.tick(time_delta);
        if enemy_spawn.spawn_timer.finished() {
            ev_spawn_enemy.send(EnemySpawnEvent {
                influence: enemy_spawn.influence,
                force_type: enemy_spawn.force_type,
                attention_span: enemy_spawn.attention_span,
                sprite: enemy_spawn.sprite.clone(),
                sprite_size: enemy_spawn.sprite_size,
                location: enemy_spawn.location,
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
    audio: Res<Audio>,
) {
    let mut rng = rand::thread_rng();

    for ev in ev_spawn_enemy.iter() {
        let texture_handle = asset_server.load(&ev.sprite);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, ev.sprite_size, 6, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let moving_delta = rng.gen_range(-25.0..25.0);
        let moving = Moving::new(Vec3::new(150.0 + moving_delta, 0.0, 0.0));

        let mut color = Color::from(ev.force_type);
        color.set_a(match ev.force_type {
            ForceType::Passive => 0.0,
            _ => 0.5,
        });

        let sound = match ev.force_type {
            ForceType::Repel => {
                let sound_a: bool = rng.gen();
                Some(asset_server.load(match sound_a {
                    true => "sounds/Manager_Mompel1.mp3",
                    false => "sounds/Manager_Mompel2.mp3",
                }))
            }
            ForceType::Attract => {
                let sound_a: bool = rng.gen();
                Some(asset_server.load(match sound_a {
                    true => "sounds/woman_gameplay1.mp3",
                    false => "sounds/woman_gameplay2.mp3",
                }))
            }
            _ => None,
        };

        if let Some(s) = sound {
            audio.play(s);
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: ev.location,
                    scale: Vec3::splat(4.0 * 1.5),
                    ..default()
                },
                ..default()
            },
            ColliderBundle {
                collider: Collider {
                    hit_box: Vec2::new(18.0 * 4.0, 20.0 * 4.0),
                    ..default()
                },
                moving,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy {
                timer: Timer::new(Duration::from_secs(ev.attention_span), TimerMode::Once),
            },
            Volatile,
            Force {
                newton: 500.0,
                influence: ev.influence,
                force_type: ev.force_type,
            },
        ));
    }
}

fn cleanup_wave(mut enemy_queue: ResMut<EnemySpawnQueue>) {
    enemy_queue.enemies.clear();
}
