use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

use crate::game::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(menu_music))
            .add_system_set(SystemSet::on_exit(GameState::Start).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(play_music))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(win_music))
            .add_system_set(SystemSet::on_exit(GameState::Won).with_system(stop));
    }
}

fn menu_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/emotinialpath.wav"))
        .looped();
}

fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/emotinialpathWithOsc.wav"))
        .looped();
}

fn win_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/emotinailpathOSCBreakout.wav"));
}

fn stop(audio: Res<Audio>) {
    audio.stop();
}
