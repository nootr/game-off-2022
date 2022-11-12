#![allow(clippy::type_complexity)]
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;
pub mod enemies;
pub mod force;
pub mod game;
pub mod level;
pub mod pathfinding;
pub mod physics;
pub mod sprite;
pub mod tower;
pub mod wave;
use camera::CameraPlugin;
use enemies::EnemyPlugin;
use force::ForcePlugin;
use game::GamePlugin;
use level::LevelPlugin;
use pathfinding::VectorFieldPlugin;
use physics::PhysicsPlugin;
use sprite::SpritePlugin;
use tower::TowerPlugin;
use wave::WavePlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(CameraPlugin)
            .add(LevelPlugin)
            .add(VectorFieldPlugin)
            .add(SpritePlugin)
            .add(TowerPlugin)
            .add(EnemyPlugin)
            .add(WavePlugin)
            .add(GamePlugin)
            .add(PhysicsPlugin)
            .add(ForcePlugin);
    }
}
