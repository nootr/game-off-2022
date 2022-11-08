use bevy::prelude::*;

use crate::physics::{Collider, Solid};
use crate::sprite::AnimationTimer;

#[derive(Component)]
pub struct Force {
    pub newton: f32,
}

pub struct ForcePlugin;

impl Plugin for ForcePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_button_input);
    }
}

fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    let window = windows.primary();

    if buttons.just_released(MouseButton::Left) {
        if let Some(raw_position) = window.cursor_position() {
            let texture_handle = asset_server.load("sprites/turret.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let position = raw_position - Vec2::new(window.width(), window.height()) / 2.0;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: position.extend(0.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(Force { newton: 10.0 })
                .insert(Collider {
                    ..Default::default()
                })
                .insert(Solid);
        }
    }
}
