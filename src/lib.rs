#![allow(clippy::type_complexity)]
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;
pub mod enemies;
pub mod force;
pub mod game;
pub mod grid;
pub mod level;
pub mod menu;
pub mod pathfinding;
pub mod physics;
pub mod sounds;
pub mod sprite;
pub mod tower;
pub mod ui;
pub mod wave;
use camera::CameraPlugin;
use enemies::EnemyPlugin;
use force::ForcePlugin;
use game::GamePlugin;
use grid::GridPlugin;
use level::LevelPlugin;
use menu::MenuPlugin;
use pathfinding::VectorFieldPlugin;
use physics::PhysicsPlugin;
use sounds::SoundPlugin;
use sprite::SpritePlugin;
use tower::TowerPlugin;
use ui::UIPlugin;
use wave::WavePlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(SoundPlugin)
            .add(UIPlugin)
            .add(MenuPlugin)
            .add(GridPlugin)
            .add(LevelPlugin)
            .add(VectorFieldPlugin)
            .add(SpritePlugin)
            .add(TowerPlugin)
            .add(EnemyPlugin)
            .add(WavePlugin)
            .add(GamePlugin)
            .add(PhysicsPlugin)
            .add(ForcePlugin)
    }
}
