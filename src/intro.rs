use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

use crate::game::{GameState, Volatile};
use crate::grid::get_coordinates;
use crate::level::setup_floor;
use crate::sprite::AnimationTimer;

#[derive(Component)]
struct Conversation;

#[derive(Component)]
struct TextView;

#[derive(Debug, Default, Resource)]
pub struct ConversationLine {
    pub number: u8,
    played_sound: bool,
}

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConversationLine::default())
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(setup_floor))
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(setup_scene))
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(setup_animations))
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(setup_textview))
            .add_system_set(SystemSet::on_update(GameState::Intro).with_system(update_conversation))
            .add_system_set(SystemSet::on_update(GameState::Intro).with_system(next_line))
            .add_system_set(SystemSet::on_exit(GameState::Intro).with_system(cleanup_textview));
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let static_objects = vec![
        // Upper wall
        (6, 15, "sprites/Cubicle_screen_corner_A.png", 1.0, true),
        (7, 15, "sprites/Cubicle_screen_square_A.png", 1.0, false),
        (8, 15, "sprites/Cubicle_screen_square_B.png", 1.0, false),
        (9, 15, "sprites/Cubicle_screen_square_B.png", 1.0, false),
        (10, 15, "sprites/Cubicle_screen_corner_A.png", 1.0, false),
        // Right wall
        (10, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
        (10, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
        (10, 15, "sprites/Cubicle_screen_side.png", 1.0, false),
        // Objects
        (9, 14, "sprites/desk_D.png", 1.5, false),
        (9, 13, "sprites/office_chair_back.png", 2.0, false),
        (8, 14, "sprites/plant_A.png", 1.0, false),
        (7, 14, "sprites/Cabinet.png", 1.5, false),
        (7, 13, "sprites/BoxB.png", 1.5, false),
        // Upper wall
        (12, 15, "sprites/Cubicle_screen_corner_B.png", 1.0, true),
        (13, 15, "sprites/Cubicle_screen_square_B.png", 1.0, false),
        (14, 15, "sprites/Cubicle_screen_square_A.png", 1.0, false),
        (15, 15, "sprites/Cubicle_screen_square_A.png", 1.0, false),
        (16, 15, "sprites/Cubicle_screen_corner_A.png", 1.0, false),
        // Left wall
        (12, 13, "sprites/Cubicle_screen_side.png", 1.0, false),
        (12, 14, "sprites/Cubicle_screen_side.png", 1.0, false),
        (12, 15, "sprites/Cubicle_screen_side.png", 1.0, false),
        // Objects
        (13, 14, "sprites/desk_C.png", 1.5, false),
        (13, 13, "sprites/office_chair.png", 2.0, false),
        (14, 14, "sprites/plant_B.png", 1.0, false),
        (14, 13, "sprites/BoxA.png", 1.5, false),
        (15, 14, "sprites/Cabinet.png", 1.5, false),
        // Upper wall
        (18, 15, "sprites/Cubicle_screen_corner_B.png", 1.0, true),
        (19, 15, "sprites/Cubicle_screen_square_B.png", 1.0, false),
        // Objects
        (19, 14, "sprites/BoxB.png", 1.5, false),
        (19, 13, "sprites/BoxA.png", 1.5, false),
        (17, 15, "sprites/archive_cabinet.png", 1.7, false),
        (5, 15, "sprites/archive_cabinet.png", 1.7, false),
        (6, 11, "sprites/Stack_of_work.png", 1.5, false),
        (17, 14, "sprites/Stack_of_work.png", 1.5, false),
        (16, 14, "sprites/Stack_of_work_B.png", 1.5, false),
    ];

    let mut z = -1.0;
    for (row, column, sprite, extra_scale, flipped) in static_objects {
        let coordinates = get_coordinates(row, column);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(sprite),
                transform: Transform {
                    translation: coordinates.extend(z),
                    rotation: match flipped {
                        true => Quat::from_rotation_y(std::f32::consts::PI),
                        false => Quat::default(),
                    },
                    scale: Vec3::splat(4.0 * extra_scale),
                },
                ..default()
            },
            Volatile,
        ));
        z += 0.01;
    }
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Bob, the elephant in the room
    let texture_handle = asset_server.load("sprites/spritesheet_elephant_side_idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0))
                .with_translation(Vec3::new(100.0, -50.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Volatile,
    ));

    // The Manager
    let texture_handle = asset_server.load("sprites/spritesheet_NPC04_M_idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0))
                .with_translation(Vec3::new(-100.0, -50.0, 0.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Volatile,
    ));
}

fn setup_textview(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    position: UiRect {
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            TextView,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..default()
                }),
                Conversation,
            ));
        });
}

fn update_conversation(
    mut conversation_line: ResMut<ConversationLine>,
    mut text_query: Query<&mut Text, With<Conversation>>,
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut text = text_query.single_mut();
    let color_bob = Color::FUCHSIA;
    let color_manager = Color::rgb(0.542, 0.674, 1.0);

    let (sound_file, color, line) = match conversation_line.number {
        0 => (Some("sounds/Manager_Intro1.mp3"), color_manager, "Look, Dumbo.."),
        1 => (Some("sounds/Bob_Intro1.mp3"), color_bob, "...it's Bob..."),
        2 => (Some("sounds/Manager_Intro2.mp3"), color_manager, "I'm only gonna tell you once."),
        3 => (Some("sounds/Manager_Intro3.mp3"), color_manager, "It wasn't my plan to hire an elephant to do\noptimisations around the department, but\nhere we are."),
        4 => (Some("sounds/Manager_Intro4.mp3"), color_manager, "As supply manager it's your job\nto optimize the department.."),
        5 => (Some("sounds/Manager_Intro2.mp3"), color_manager, "..but DON'T cause any distractions!"),
        6 => (Some("sounds/Manager_Intro5.mp3"), color_manager, "If I see a co-worker hanging around\nat your station you will be fired."),
        7 => (Some("sounds/Manager_Intro6.mp3"), color_manager, "I don't care how you do it but make\nsure to hide that face of yours."),
        8 => (Some("sounds/Manager_Intro7.mp3"), color_manager, "Employees here aren't accustomed\nseeing elephants everyday.."),
        9 => (Some("sounds/Manager_Intro8.mp3"), color_manager, "..and it would make my day a lot\nbetter if I don't see that dopey\nface of yours the entire day."),
        10 => (Some("sounds/Manager_Intro1.mp3"), color_manager, "Enjoy your workday, Dumbo!"),
        11 => (None, color_bob, "..."),
        _ => {
            game_state.set(GameState::Start).unwrap();
            (None, Color::RED, "???")
        }
    };

    text.sections[0].value = line.to_string();
    text.sections[0].style.color = color;

    if let Some(file) = sound_file {
        if !conversation_line.played_sound {
            audio.play(asset_server.load(file));
            conversation_line.played_sound = true;
        }
    }
}

fn next_line(buttons: Res<Input<MouseButton>>, mut conversation_line: ResMut<ConversationLine>) {
    if buttons.just_pressed(MouseButton::Left) {
        conversation_line.number += 1;
        conversation_line.played_sound = false;
    }
}

fn cleanup_textview(mut commands: Commands, textview_query: Query<Entity, With<TextView>>) {
    let textview = textview_query.single();
    commands.entity(textview).despawn_recursive();
}
