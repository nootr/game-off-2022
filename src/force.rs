use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::{GameState, Volatile};
use crate::physics::{Collider, Solid};
use crate::sprite::AnimationTimer;
use crate::ui::UIBar;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ForceType {
    Passive,
    Attract,
    Repel,
}

#[derive(Component)]
pub struct Force {
    pub newton: f32,
    pub influence: f32,
    pub force_type: ForceType,
}

impl Force {
    pub fn get_force(&self, position: Vec2, force_position: Vec2) -> Option<Vec2> {
        let vector = match self.force_type {
            ForceType::Attract => force_position - position,
            ForceType::Repel => position - force_position,
            ForceType::Passive => {
                return None;
            }
        };

        let distance = vector.length() / 4.0;

        if 0.0 < distance && distance < self.influence {
            Some(vector.normalize() * self.newton)
        } else {
            None
        }
    }
}

pub struct ForcePlugin;

impl Plugin for ForcePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(mouse_button_input));
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
    uibar_query: Query<(&UIBar, &Node), With<UIBar>>,
) {
    let (uibar, uibar_node) = uibar_query.single();

    let window = windows.primary();
    let window_width = window.width();
    let window_height = window.height();

    if buttons.just_released(MouseButton::Left) {
        if let Some(raw_position) = window.cursor_position() {
            let texture_handle = asset_server.load("sprites/turret.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let position = raw_position - Vec2::new(window_width, window_height) / 2.0;

            if raw_position.x < uibar_node.size().x {
                // Do not summon a force within the UI bar.
                return;
            }

            let influence = 50.0;
            let force_type = uibar.selected_force;

            let force_field = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(influence))).into(),
                    material: materials.add(ColorMaterial::from(match force_type {
                        ForceType::Passive => Color::rgba(0.0, 0.0, 0.0, 0.0),
                        ForceType::Attract => Color::rgba(1.0, 0.0, 0.0, 0.5),
                        ForceType::Repel => Color::rgba(0.0, 0.0, 1.0, 0.5),
                    })),
                    ..default()
                })
                .insert(Volatile)
                .id();

            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: position.extend(-1.0),
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
                    force_type,
                })
                .insert(Collider::default())
                .insert(Solid)
                .insert(Volatile)
                .push_children(&[force_field]);
        }
    }
}
