use bevy::prelude::*;

use crate::game::GameState;
use crate::grid::snap;

#[derive(Component)]
pub struct Ghost;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(move_ghost))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(cleanup_ghosts))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup_ghosts));
    }
}

fn move_ghost(mut ghost_query: Query<&mut Transform, With<Ghost>>, windows: Res<Windows>) {
    for mut transform in &mut ghost_query {
        let window = windows.primary();
        let window_width = window.width();
        let window_height = window.height();

        if let Some(raw_position) = window.cursor_position() {
            let position = raw_position - Vec2::new(window_width, window_height) / 2.0;

            transform.translation = snap(position).extend(0.0);
        }
    }
}

fn cleanup_ghosts(mut commands: Commands, ghost_query: Query<Entity, With<Ghost>>) {
    for entity in &ghost_query {
        commands.entity(entity).despawn();
    }
}
