//! Loads and renders a glTF file as a scene.

use bevy::math::f32::Quat;
use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Camera;

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 2.0, -2.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Camera);
    commands.spawn_bundle(
        TextBundle::from_section(
            "Game Off\n2022",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(100.0),
                bottom: Val::Px(100.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.08,
            ..default()
        }),
        ..default()
    });
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform), Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
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

    for mut camera_transform in &mut camera_query {
        camera_transform.rotation = camera_transform
            .looking_at(transform.translation, Vec3::Y)
            .rotation;
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
