use bevy::prelude::*;
use std::time::Duration;

use crate::level::Level;
use crate::sprite::AnimationTimer;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Start,
    Intro,
    InGame,
    Won,
    GameOver,
}

#[derive(Component)]
struct StartText;

#[derive(Component)]
struct TimeText {
    timer: Timer,
}

#[derive(Component)]
struct StateTimer {
    timer: Timer,
}

/// Volatile entities are despawned after the GameOver state exits.
#[derive(Component)]
pub struct Volatile;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainMenu)
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver).with_system(set_game_over_timer),
            )
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(tick_state_timer))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup_volatile))
            .add_system_set(
                SystemSet::on_enter(GameState::Start)
                    .with_system(setup_start_text)
                    .with_system(cleanup_volatile),
            )
            .add_system_set(SystemSet::on_update(GameState::Start).with_system(update_timer_text))
            .add_system_set(SystemSet::on_exit(GameState::Start).with_system(cleanup_start))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(cleanup_volatile))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(next_level))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(show_win_animation))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(set_win_timer))
            .add_system_set(SystemSet::on_update(GameState::Won).with_system(tick_state_timer));
    }
}

fn cleanup_volatile(mut commands: Commands, volatile_query: Query<Entity, With<Volatile>>) {
    for entity in &volatile_query {
        commands.entity(entity).despawn();
    }
}

fn set_game_over_timer(mut commands: Commands) {
    commands.spawn(StateTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
    });
}

fn set_win_timer(mut commands: Commands) {
    commands.spawn(StateTimer {
        timer: Timer::new(Duration::from_secs(7), TimerMode::Once),
    });
}

fn next_level(mut level: ResMut<Level>) {
    level.level += 1;
}

fn tick_state_timer(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut q: Query<(Entity, &mut StateTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in q.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.entity(entity).despawn();
            game_state.set(GameState::Start).unwrap();
        }
    }
}

fn setup_start_text(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            },
            StartText,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    level.title(),
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                }),
            );

            parent.spawn((
                TextBundle::from_section(
                    "08:59",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
                TimeText {
                    timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
                },
            ));

            if let Some(help_text) = level.help_text() {
                parent.spawn((TextBundle::from_section(
                    format!("Hint: {}", help_text),
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 40.0,
                        color: Color::GRAY,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(150.0),
                        ..default()
                    },
                    ..default()
                }),));
            }
        });
}

fn update_timer_text(
    mut text_query: Query<(&mut Text, &mut TimeText)>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (mut text, mut time_text) = text_query.single_mut();

    time_text.timer.tick(time.delta());

    if time_text.timer.finished() {
        game_state.set(GameState::InGame).unwrap();
    } else if time_text.timer.elapsed_secs() < 3.5 {
        text.sections[0].value = "08:59 AM".to_string();
    } else {
        text.sections[0].value = "09:00 AM".to_string();
    }
}

fn cleanup_start(mut commands: Commands, start_text_query: Query<Entity, With<StartText>>) {
    let start_text = start_text_query.single();
    commands.entity(start_text).despawn_recursive();
}

fn show_win_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/elephant_tram_animation.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 72.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(10.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Volatile,
    ));
}
