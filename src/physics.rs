use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

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
            hit_box: Vec2::new(4.0 * 24.0, 4.0 * 24.0),
        }
    }
}

#[derive(Component, Default)]
pub struct Moving {
    pub velocity: Vec3,
    pub speed: f32,
}

impl Moving {
    pub fn new(velocity: Vec3) -> Self {
        Moving {
            velocity,
            speed: velocity.length(),
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
        app.add_system(collision_system).add_system(move_system);
    }
}

fn collision_system(
    mut solid_query: Query<(&mut Collider, &Transform), With<Solid>>,
    mut collider_query: Query<(&mut Collider, &mut Moving, &mut Transform), Without<Solid>>,
    time: Res<Time>,
) {
    for (mut collider, mut moving, mut collider_transform) in &mut collider_query {
        for (mut solid_collider, solid_transform) in &mut solid_query {
            if let Some(collision) = collide(
                collider_transform.translation,
                collider.hit_box,
                solid_transform.translation,
                solid_collider.hit_box,
            ) {
                if !collider.hit {
                    collider_transform.translation = collider_transform.translation
                        - moving.velocity.normalize() * moving.speed * time.delta_seconds();
                }

                solid_collider.hit = true;
                collider.hit = true;

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
            } else {
                collider.hit = false;
            }
        }
    }
}

fn move_system(mut query: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (moving, mut transform) in &mut query {
        transform.translation += moving.velocity * time.delta_seconds();
    }
}
