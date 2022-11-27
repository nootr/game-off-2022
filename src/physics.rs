use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use log::warn;

use crate::game::GameState;
use crate::grid::{get_coordinates, get_indeces};

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Collider {
    pub hit: bool,
    pub hit_box: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            hit: false,
            hit_box: Vec2::new(4.0 * 20.0, 4.0 * 20.0),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum MovingState {
    Normal,
    Retrace,
}

#[derive(Component)]
pub struct Moving {
    pub state: MovingState,
    pub velocity: Vec3,
    pub speed: f32,
    pub route_history: Vec<(usize, usize)>,
    last_delta: Option<Vec3>,
}

impl Default for Moving {
    fn default() -> Self {
        Moving {
            velocity: Vec3::X,
            speed: 0.0,
            last_delta: None,
            route_history: Vec::new(),
            state: MovingState::Normal,
        }
    }
}

impl Moving {
    pub fn new(velocity: Vec3) -> Self {
        Moving {
            velocity,
            speed: velocity.length(),
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub moving: Moving,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(collision_system))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(move_system));
    }
}

fn collision_system(
    mut solid_query: Query<(&mut Collider, &Transform), With<Solid>>,
    mut collider_query: Query<(&mut Collider, &mut Moving, &mut Transform), Without<Solid>>,
) {
    for (mut collider, mut moving, mut collider_transform) in &mut collider_query {
        for (mut solid_collider, solid_transform) in &mut solid_query {
            if let Some(collision) = collide(
                collider_transform.translation,
                collider.hit_box,
                solid_transform.translation,
                solid_collider.hit_box,
            ) {
                if collider.hit {
                    warn!("Consecutive hits!");
                } else if let Some(delta) = moving.last_delta {
                    collider_transform.translation -= delta;
                }

                solid_collider.hit = true;
                collider.hit = true;

                let (reflect_x, reflect_y) = match collision {
                    Collision::Left => (moving.velocity.x > 0.0, false),
                    Collision::Right => (moving.velocity.x < 0.0, false),
                    Collision::Bottom => (false, moving.velocity.y > 0.0),
                    Collision::Top => (false, moving.velocity.y < 0.0),
                    Collision::Inside => (false, false),
                };

                if reflect_x {
                    moving.velocity.x = -moving.velocity.x;
                }

                if reflect_y {
                    moving.velocity.y = -moving.velocity.y;
                }
            } else {
                collider.hit = false;
            }
        }
    }
}

fn move_system(mut query: Query<(&mut Moving, &mut Transform)>, time: Res<Time>) {
    for (mut moving, mut transform) in &mut query {
        match moving.state {
            MovingState::Normal => {
                let delta = moving.velocity.normalize() * moving.speed * time.delta_seconds();
                moving.last_delta = Some(delta);
                transform.translation += delta;

                let indeces = get_indeces(transform.translation.truncate());
                match moving.route_history.last() {
                    Some(last_indeces) => {
                        if *last_indeces != indeces {
                            moving.route_history.push(indeces);
                        }
                    }
                    None => moving.route_history.push(indeces),
                }
            }
            MovingState::Retrace => {
                // Remove last indeces from route history if we've arrived there.
                if let Some(last_indeces) = moving.route_history.last() {
                    let current_indeces = get_indeces(transform.translation.truncate());
                    if current_indeces == *last_indeces {
                        moving.route_history.pop();
                    }
                }

                let delta = match moving.route_history.last() {
                    Some((x, y)) => {
                        let current_coordinates = get_coordinates(*x, *y);
                        current_coordinates.extend(0.0) - transform.translation
                    }
                    None => moving.last_delta.unwrap_or(Vec3::ZERO),
                }
                .normalize()
                    * moving.speed
                    * time.delta_seconds();

                moving.last_delta = Some(delta);
                transform.translation += delta;
            }
        }
    }
}
