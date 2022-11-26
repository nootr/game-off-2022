use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::camera::CameraShake;
use crate::game::{GameState, Volatile};
use crate::grid::snap;
use crate::physics::{Collider, Solid};
use crate::sprite::AnimationTimer;
use crate::ui::UIBar;

const PASSIVE_COLOR: Color = Color::rgb(0.0, 0.65, 0.0);
const ATTRACT_COLOR: Color = Color::rgb(0.65, 0.0, 0.0);
const REPEL_COLOR: Color = Color::rgb(0.0, 0.0, 0.65);

struct ForceSpawnEvent {
    position: Vec2,
    force_type: ForceType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ForceType {
    Passive,
    Attract,
    Repel,
}

impl From<ForceType> for Color {
    fn from(force_type: ForceType) -> Self {
        match force_type {
            ForceType::Passive => PASSIVE_COLOR,
            ForceType::Attract => ATTRACT_COLOR,
            ForceType::Repel => REPEL_COLOR,
        }
    }
}

impl ForceType {
    pub fn price(&self) -> f32 {
        match self {
            ForceType::Passive => 30.0,
            ForceType::Attract => 50.0,
            ForceType::Repel => 60.0,
        }
    }
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
        app.add_event::<ForceSpawnEvent>()
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(mouse_button_input))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_force));
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    mut ev_spawn_force: EventWriter<ForceSpawnEvent>,
    windows: Res<Windows>,
    mut uibar_query: Query<(&mut UIBar, &Node), With<UIBar>>,
) {
    let (mut uibar, uibar_node) = uibar_query.single_mut();
    let window = windows.primary();
    let window_width = window.width();
    let window_height = window.height();

    if buttons.just_released(MouseButton::Left) {
        if let Some(force_type) = uibar.selected_force {
            if let Some(raw_position) = window.cursor_position() {
                if raw_position.x < uibar_node.size().x {
                    // Do not summon a force within the UI bar.
                    return;
                }

                let position = raw_position - Vec2::new(window_width, window_height) / 2.0;

                ev_spawn_force.send(ForceSpawnEvent {
                    position: snap(position),
                    force_type,
                });

                uibar.selected_force = None;
            }
        }
    }
}

fn spawn_force(
    mut commands: Commands,
    mut ev_spawn_force: EventReader<ForceSpawnEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera_query: Query<&mut CameraShake>,
) {
    let influence = 50.0;

    for ev in ev_spawn_force.iter() {
        let mut shake = camera_query.single_mut();
        shake.trauma += 0.3;

        let mut color = Color::from(ev.force_type);
        color.set_a(match ev.force_type {
            ForceType::Passive => 0.0,
            _ => 0.5,
        });

        let texture_handle = asset_server.load("sprites/turret.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let force_field = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(influence))).into(),
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                },
                Volatile,
            ))
            .id();

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: ev.position.extend(-1.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Force {
                    newton: 500.0,
                    influence,
                    force_type: ev.force_type,
                },
                Collider::default(),
                Solid,
                Volatile,
            ))
            .push_children(&[force_field]);
    }
}
