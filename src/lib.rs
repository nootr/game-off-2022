#![allow(clippy::type_complexity)]
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod audio;
pub mod camera;
pub mod cost;
pub mod enemies;
pub mod force;
pub mod game;
pub mod grid;
pub mod intro;
pub mod level;
pub mod menu;
pub mod pathfinding;
pub mod physics;
pub mod sprite;
pub mod tower;
pub mod ui;
pub mod wave;
use audio::AudioPlugin;
use camera::CameraPlugin;
use cost::CostPlugin;
use enemies::EnemyPlugin;
use force::ForcePlugin;
use game::GamePlugin;
use grid::GridPlugin;
use intro::IntroPlugin;
use level::LevelPlugin;
use menu::MenuPlugin;
use pathfinding::VectorFieldPlugin;
use physics::PhysicsPlugin;
use sprite::SpritePlugin;
use tower::TowerPlugin;
use ui::UIPlugin;
use wave::WavePlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(AudioPlugin)
            .add(UIPlugin)
            .add(MenuPlugin)
            .add(IntroPlugin)
            .add(GridPlugin)
            .add(CostPlugin)
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
