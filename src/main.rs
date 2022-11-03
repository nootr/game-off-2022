use bevy::{prelude::*, render::texture::ImageSettings};

mod enemies;
mod game;
mod physics;
mod sprite;
mod tower;
use enemies::EnemySpawnerPlugin;
use game::GamePlugin;
use physics::PhysicsPlugin;
use sprite::SpritePlugin;
use tower::TowerPlugin;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_plugin(SpritePlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(EnemySpawnerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
