use bevy::prelude::*;

use crate::game::GameState;

#[derive(Component, Default)]
pub struct MainMenuRoot {
    frame: f32,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(show_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(mouse_button_input),
            )
            .add_system_set(SystemSet::on_update(GameState::Start).with_system(scroll_down))
            .add_system_set(SystemSet::on_exit(GameState::Start).with_system(cleanup_root));
    }
}

fn show_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            },
            MainMenuRoot::default(),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Trumpet Trainee",
                    TextStyle {
                        font: asset_server.load("fonts/Franchise.ttf"),
                        font_size: 150.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                }),
            );

            parent.spawn(
                TextBundle::from_section(
                    "Click to start",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        });
}

fn mouse_button_input(mut game_state: ResMut<State<GameState>>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_released(MouseButton::Left) {
        game_state.set(GameState::Start).unwrap();
    }
}

fn cleanup_root(mut commands: Commands, menu_query: Query<Entity, With<MainMenuRoot>>) {
    let main_menu_root = menu_query.single();
    commands.entity(main_menu_root).despawn_recursive();
}

fn scroll_down(
    mut game_state: ResMut<State<GameState>>,
    mut style_query: Query<(&mut Style, &mut MainMenuRoot)>,
    time: Res<Time>,
) {
    let (mut style, mut root) = style_query.single_mut();

    style.position.top = Val::Percent(root.frame);
    root.frame += 75.0 * time.delta_seconds();

    if root.frame > 100.0 {
        game_state.set(GameState::InGame).unwrap();
    }
}
