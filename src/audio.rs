use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

use crate::game::GameState;
use crate::level::Level;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(menu_music))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(intro_music))
            .add_system_set(SystemSet::on_exit(GameState::Intro).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(play_music))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(lose_music))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::Won).with_system(win_music))
            .add_system_set(SystemSet::on_exit(GameState::Won).with_system(stop))
            .add_system_set(SystemSet::on_enter(GameState::End).with_system(end_music))
            .add_system_set(SystemSet::on_exit(GameState::End).with_system(stop));
    }
}

fn menu_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/emotinialpath.wav"))
        .with_volume(0.5)
        .looped();
}

fn intro_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/condensOnACoffeeMuck.wav"))
        .with_volume(0.5)
        .looped();
}

fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>, level: Res<Level>) {
    audio
        .play(asset_server.load(match level.level {
            1 => "sounds/emotinialpathWithOsc.wav",
            2 => "sounds/condensOnACoffeeMuck.wav",
            3 => "sounds/ok_agan.wav",
            4 => "sounds/all_hands_on_deck.wav",
            _ => "sounds/emotinialpathWithOsc.wav",
        }))
        .with_volume(0.5)
        .looped();
}

fn win_music(asset_server: Res<AssetServer>, audio: Res<Audio>, level: Res<Level>) {
    audio
        .play(asset_server.load(match level.level {
            1 => "sounds/emotinailpathOSCBreakout.wav",
            2 => "sounds/condensOnACoffeeMuckpiano.wav",
            3 => "sounds/ok_agan_win.wav",
            4 => "sounds/all_hands_on_deck_win.wav",
            _ => "sounds/emotinailpathOSCBreakout.wav",
        }))
        .with_volume(0.5)
        .looped();
}

fn lose_music(asset_server: Res<AssetServer>, audio: Res<Audio>, level: Res<Level>) {
    audio
        .play(asset_server.load(match level.level {
            1 => "sounds/hide_paneel_lose.wav",
            2 => "sounds/condensOnACoffeeMuckLose.wav",
            3 => "sounds/ok_agan_lose.wav",
            4 => "sounds/all_hands_on_deck_lose.wav",
            _ => "sounds/hide_paneel_lose.wav",
        }))
        .with_volume(0.5)
        .looped();
}

fn end_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/emotinialpathWithOscUnfilterloop.wav"))
        .with_volume(0.5)
        .looped();
}

fn stop(audio: Res<Audio>) {
    audio.stop();
}
