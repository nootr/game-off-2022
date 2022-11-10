use bevy::{app::PluginGroupBuilder, prelude::*};

mod camera;
mod enemies;
mod force;
mod game;
mod level;
mod pathfinding;
mod physics;
mod sprite;
mod tower;
use camera::CameraPlugin;
use enemies::EnemySpawnerPlugin;
use force::ForcePlugin;
use game::GamePlugin;
use level::LevelPlugin;
use pathfinding::VectorFieldPlugin;
use physics::PhysicsPlugin;
use sprite::SpritePlugin;
use tower::TowerPlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(CameraPlugin)
            .add(LevelPlugin)
            .add(VectorFieldPlugin)
            .add(SpritePlugin)
            .add(TowerPlugin)
            .add(EnemySpawnerPlugin)
            .add(GamePlugin)
            .add(PhysicsPlugin)
            .add(ForcePlugin);
    }
}
