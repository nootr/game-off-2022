use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::Volatile;
use crate::physics::{Collider, Solid};
use crate::sprite::AnimationTimer;

#[derive(Component)]
pub struct Force {
    pub newton: f32,
    pub influence: f32,
}

impl Force {
    pub fn get_force(&self, position: Vec3, force_position: Vec3) -> Option<Vec3> {
        let vector = force_position - position;
        let distance = vector.length() / 4.0;

        if distance < self.influence {
            return Some(vector.normalize() * self.newton);
        }

        None
    }
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.primary();

    if buttons.just_released(MouseButton::Left) {
        if let Some(raw_position) = window.cursor_position() {
            let texture_handle = asset_server.load("sprites/turret.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let position = raw_position - Vec2::new(window.width(), window.height()) / 2.0;

            let influence = 50.0;

            let force_field = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(influence))).into(),
                    material: materials.add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.5))),
                    ..default()
                })
                .insert(Volatile)
                .id();

            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: position.extend(0.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    0.1,
                    TimerMode::Repeating,
                )))
                .insert(Force {
                    newton: 500.0,
                    influence,
                })
                .insert(Collider { ..default() })
                .insert(Solid)
                .insert(Volatile)
                .push_children(&[force_field]);
        }
    }
}
