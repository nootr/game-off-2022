//! A Bevy plugin for a camera with a shake feature.
//!
//! The shaking algorithm is based on [an excellent presentation by Squirrel
//! Eiserloh](https://www.youtube.com/watch?v=tu-Qe66AvtY).
//!

use bevy::{prelude::*, render::texture::ImageSettings};
use rand::Rng;

#[derive(Debug, Component)]
pub struct CameraShake {
    pub trauma: f32,
    max_angle: f32,
    max_offset: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        CameraShake {
            trauma: 0.0,
            max_angle: 10.0_f32.to_radians(),
            max_offset: 10.0,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
            .add_startup_system(setup_camera)
            .add_system(shake_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(CameraShake::default());
}

fn shake_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraShake, &mut Transform), With<Camera>>,
) {
    let (mut shake, mut transform) = camera_query.single_mut();
    let mut rng = rand::thread_rng();

    if shake.trauma < 0.0 {
        shake.trauma = 0.0;
    }
    if shake.trauma > 1.0 {
        shake.trauma = 1.0;
    }

    transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        shake.max_angle * shake.trauma * shake.trauma * rng.gen_range(-1.0..1.0),
    );

    transform.translation = Vec3::new(
        shake.max_offset * shake.trauma * shake.trauma * rng.gen_range(-1.0..1.0),
        shake.max_offset * shake.trauma * shake.trauma * rng.gen_range(-1.0..1.0),
        0.0,
    );

    shake.trauma -= time.delta_seconds() * 1.0;
}
