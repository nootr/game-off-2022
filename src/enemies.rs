use bevy::prelude::*;

use crate::force::Force;
use crate::pathfinding::VectorField;
use crate::physics::Moving;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(turn_enemy);
    }
}

fn turn_enemy(
    mut force_query: Query<(&Force, &Transform)>,
    mut enemy_query: Query<(&mut Moving, &Transform), With<Enemy>>,
    vector_field: Res<VectorField>,
    time: Res<Time>,
) {
    for (mut moving, transform) in &mut enemy_query {
        // Slowly point enemy towards tower
        let mut force_sum =
            vector_field.get_direction(transform.translation.truncate()) * moving.speed.abs();

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
