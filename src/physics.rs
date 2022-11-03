use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::tower::Tower;

#[derive(Component)]
pub struct Collider {
    pub hit: bool,
    pub hit_box: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            hit: false,
            hit_box: Vec2::new(1.0, 1.0),
        }
    }
}

#[derive(Component, Default)]
pub struct Moving {
    pub velocity: f32,
}

#[derive(Bundle, Default)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub moving: Moving,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system).add_system(move_system);
    }
}

fn collision_system(
    mut tower_query: Query<(&mut Collider, &Transform), With<Tower>>,
    mut collider_query: Query<(&mut Collider, &Transform), Without<Tower>>,
) {
    let (mut tower_collider, tower_transform) = tower_query.single_mut();

    for (mut collider, collider_transform) in &mut collider_query {
        if collide(
            collider_transform.translation,
            collider.hit_box,
            tower_transform.translation,
            tower_collider.hit_box,
        )
        .is_some()
        {
            tower_collider.hit = true;
            collider.hit = true;
        }
    }
}

fn move_system(mut query: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (moving, mut transform) in &mut query {
        transform.translation =
            transform.translation + transform.right() * moving.velocity * time.delta_seconds();
    }
}
