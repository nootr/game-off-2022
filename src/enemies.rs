use bevy::prelude::*;

use crate::force::Force;
use crate::pathfinding::VectorField;
use crate::physics::Moving;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum EnemyState {
    #[default]
    WalkToTower,
    WalkBack,
}

#[derive(Component, Default)]
pub struct Enemy {
    /// Tracks when enemy should turn around and leave.
    pub timer: Timer,
    pub state: EnemyState,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(turn_enemy).add_system(update_timer);
    }
}

fn turn_enemy(
    mut force_query: Query<(&Force, &Transform)>,
    mut enemy_query: Query<(&Enemy, &mut Moving, &Transform)>,
    vector_field: Res<VectorField>,
    time: Res<Time>,
) {
    for (enemy, mut moving, transform) in &mut enemy_query {
        // Slowly point enemy towards tower
        let mut force_sum = match enemy.state {
            EnemyState::WalkToTower => {
                vector_field.get_direction(transform.translation.truncate()) * moving.speed.abs()
            }
            EnemyState::WalkBack => Vec2::ZERO,
        };

        for (force, force_transform) in &mut force_query {
            if let Some(f) = force.get_force(
                transform.translation.truncate(),
                force_transform.translation.truncate(),
            ) {
                force_sum += f;
            }
        }

        let turning_speed = time.delta_seconds() * 5000.0;
        moving.velocity =
            ((turning_speed - 1.0) * moving.velocity + force_sum.extend(0.0)) / turning_speed;
    }
}

fn update_timer(mut enemy_query: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in enemy_query.iter_mut() {
        if enemy.state == EnemyState::WalkToTower {
            enemy.timer.tick(time.delta());

            if enemy.timer.finished() {
                enemy.state = EnemyState::WalkBack;
            }
        }
    }
}
