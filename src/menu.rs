use bevy::prelude::*;

use crate::game::GameState;

#[derive(Component)]
pub struct MainMenuRoot;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(show_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(mouse_button_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup_root));
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
            MainMenuRoot,
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
        game_state.set(GameState::Intro).unwrap();
    }
}

fn cleanup_root(mut commands: Commands, menu_query: Query<Entity, With<MainMenuRoot>>) {
    let main_menu_root = menu_query.single();
    commands.entity(main_menu_root).despawn_recursive();
}
