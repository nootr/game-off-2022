use bevy::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameState {
    InGame,
    GameOver,
}

#[derive(Component)]
struct GameOverTimer {
    timer: Timer,
}

/// Volatile entities are despawned after the GameOver state exits.
#[derive(Component)]
pub struct Volatile;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::InGame)
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver).with_system(set_game_over_timer),
            )
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(tick_game_over_timer),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup_volatile));
    }
}

fn cleanup_volatile(mut commands: Commands, volatile_query: Query<Entity, With<Volatile>>) {
    for entity in &volatile_query {
        commands.entity(entity).despawn();
    }
}

fn set_game_over_timer(mut commands: Commands) {
    commands.spawn().insert(GameOverTimer {
        timer: Timer::new(Duration::from_secs(1), false),
    });
}

fn tick_game_over_timer(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut q: Query<(Entity, &mut GameOverTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in q.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.entity(entity).despawn();
            game_state.set(GameState::InGame).unwrap();
        }
    }
}
