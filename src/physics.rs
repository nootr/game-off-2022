use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::tower::Tower;

#[derive(Component)]
pub struct Collider {
    pub hit: bool,
    pub hit_box: Vec2,
    pub solid: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            hit: false,
            hit_box: Vec2::new(1.0, 1.0),
            solid: false,
        }
    }
}

#[derive(Component, Default)]
pub struct Moving {
    pub velocity: Vec3,
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
    mut collider_query: Query<(&mut Collider, &mut Moving, &mut Transform), Without<Tower>>,
    time: Res<Time>,
) {
    let (mut tower_collider, tower_transform) = tower_query.single_mut();

    for (mut collider, mut moving, mut collider_transform) in &mut collider_query {
        if let Some(collision) = collide(
            collider_transform.translation,
            collider.hit_box,
            tower_transform.translation,
            tower_collider.hit_box,
        ) {
            tower_collider.hit = true;
            collider.hit = true;

            collider_transform.translation =
                collider_transform.translation - moving.velocity * time.delta_seconds();

            if tower_collider.solid
            // TODO: generalize solid colliders
            {
                let (reflect_x, reflect_y) = match collision {
                    Collision::Left => (moving.velocity.x > 0.0, false),
                    Collision::Right => (moving.velocity.x < 0.0, false),
                    Collision::Top => (false, moving.velocity.y < 0.0),
                    Collision::Bottom => (false, moving.velocity.y > 0.0),
                    Collision::Inside => (moving.velocity.x < 0.0, moving.velocity.y < 0.0),
                };

                if reflect_x {
                    moving.velocity.x = -moving.velocity.x;
                }
                if reflect_y {
                    moving.velocity.y = -moving.velocity.y;
                }
            }
        }
    }
}

fn move_system(mut query: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (moving, mut transform) in &mut query {
        transform.translation += moving.velocity * time.delta_seconds();
    }
}
