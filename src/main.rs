//! Loads and renders a glTF file as a scene.

use bevy::math::f32::Quat;
use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct ColorText;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(7.0, 7.0, 10.0)
            .looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y),
        ..default()
    });
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Game Off 2022",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::YELLOW,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(ColorText);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -1.0,
                right: 1.0,
                bottom: -1.0,
                top: 1.0,
                near: -10.0,
                far: 10.0,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("models/Duck.gltf#Scene0"),
            ..default()
        })
        .insert(Player { speed: 5.0 });
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut forward_delta = 0.0;
    if keyboard.pressed(KeyCode::E) {
        forward_delta -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        forward_delta += player.speed * time.delta_seconds();
    }

    let mut left_delta = 0.0;
    if keyboard.pressed(KeyCode::S) {
        left_delta += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        left_delta -= player.speed * time.delta_seconds();
    }

    transform.translation =
        transform.translation + transform.left() * left_delta + transform.forward() * forward_delta;

    if keyboard.pressed(KeyCode::A) {
        transform.rotate_local_y(player.speed * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::D) {
        transform.rotate_local_y(-player.speed * time.delta_seconds());
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}
