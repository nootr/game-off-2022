use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::prelude::{Audio, *};
use rand::Rng;

use crate::camera::CameraShake;
use crate::cost::Points;
use crate::game::{GameState, Volatile};
use crate::ghost::Ghost;
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
            ForceType::Repel => 45.0,
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
    solid_query: Query<(&Collider, &Transform), With<Solid>>,
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

                // Prevent player from placing on a solid object
                for (solid_collider, solid_transform) in &solid_query {
                    if collide(
                        solid_transform.translation,
                        solid_collider.hit_box,
                        position.extend(0.0),
                        Vec2::new(1.0, 1.0),
                    )
                    .is_some()
                    {
                        return;
                    }
                }

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
    mut points: ResMut<Points>,
    mut ev_spawn_force: EventReader<ForceSpawnEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut camera_query: Query<&mut CameraShake>,
    ghost_query: Query<Entity, With<Ghost>>,
    audio: Res<Audio>,
) {
    let mut rng = rand::thread_rng();
    let influence = 50.0;

    for ev in ev_spawn_force.iter() {
        for entity in &ghost_query {
            commands.entity(entity).despawn();
        }

        points.owned -= ev.force_type.price();

        let mut shake = camera_query.single_mut();
        shake.trauma += 0.3;

        let mut color = Color::from(ev.force_type);
        color.set_a(match ev.force_type {
            ForceType::Passive => 0.0,
            _ => 0.5,
        });

        let (sound, texture_atlas) = match ev.force_type {
            ForceType::Passive => {
                let render_box_a: bool = rng.gen();
                let texture_handle = asset_server.load(match render_box_a {
                    true => "sprites/BoxA.png",
                    false => "sprites/BoxB.png",
                });
                let sound = asset_server.load("sounds/doos1.mp3");
                (
                    Some(sound),
                    TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(16.0, 16.0),
                        1,
                        1,
                        None,
                        None,
                    ),
                )
            }
            ForceType::Attract => {
                let texture_handle = asset_server.load("sprites/spritesheet_coffee.png");
                let sound_a: bool = rng.gen();
                let sound = asset_server.load(match sound_a {
                    true => "sounds/koffie_gameplay1.mp3",
                    false => "sounds/koffie_gameplay2.mp3",
                });
                (
                    Some(sound),
                    TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(16.0, 16.0),
                        6,
                        1,
                        None,
                        None,
                    ),
                )
            }
            ForceType::Repel => {
                let render_box_a: bool = rng.gen();
                let texture_handle = asset_server.load(match render_box_a {
                    true => "sprites/Stack_of_work.png",
                    false => "sprites/Stack_of_work_B.png",
                });
                let sound = asset_server.load("sounds/stapelwerk_gameplay1.mp3");
                (
                    Some(sound),
                    TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(16.0, 16.0),
                        1,
                        1,
                        None,
                        None,
                    ),
                )
            }
        };

        if let Some(s) = sound {
            audio.play(s);
        }

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: ev.position.extend(-1.0),
                    scale: Vec3::splat(4.0 * 1.5),
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
        ));
    }
}
