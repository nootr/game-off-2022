use bevy::prelude::*;

use crate::force::ForceType;
use crate::game::GameState;

const PASSIVE_COLOR: Color = Color::rgb(0.0, 0.65, 0.0);
const ATTRACT_COLOR: Color = Color::rgb(0.65, 0.0, 0.0);
const REPEL_COLOR: Color = Color::rgb(0.0, 0.0, 0.65);

#[derive(Component)]
pub struct UIBar {
    pub selected_force: ForceType,
}

#[derive(Component)]
struct ForceButton {
    force_type: ForceType,
    hovered: bool,
}

impl ForceButton {
    fn base_color(&self) -> Color {
        match self.force_type {
            ForceType::Passive => PASSIVE_COLOR,
            ForceType::Attract => ATTRACT_COLOR,
            ForceType::Repel => REPEL_COLOR,
        }
    }

    fn color(&self, uibar: &mut UIBar) -> Color {
        let mut color = self.base_color();
        color.set_a(self.alpha(&uibar));
        color
    }

    fn alpha(&self, uibar: &UIBar) -> f32 {
        if self.hovered {
            0.6
        } else if self.selected(uibar) {
            1.0
        } else {
            0.4
        }
    }

    fn selected(&self, uibar: &UIBar) -> bool {
        self.force_type == uibar.selected_force
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(click_button))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(button_color));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.primary();
    let bar_width = window.width() / 12.0;
    let bar_height = window.height();

    fn create_ui_button(width: f32, height: f32, background_color: Color) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: background_color.into(),
            ..default()
        }
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(bar_width), Val::Percent(bar_height)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (window.width() - bar_width) / 2.0,
                0.0,
                0.0,
            )),
            ..default()
        })
        .with_children(|bar| {
            bar.spawn(create_ui_button(bar_width, bar_height / 3.0, PASSIVE_COLOR))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Passive",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(ForceButton {
                    force_type: ForceType::Passive,
                    hovered: false,
                });

            bar.spawn(create_ui_button(bar_width, bar_height / 3.0, ATTRACT_COLOR))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Attract",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(ForceButton {
                    force_type: ForceType::Attract,
                    hovered: false,
                });

            bar.spawn(create_ui_button(bar_width, bar_height / 3.0, REPEL_COLOR))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Repel",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(ForceButton {
                    force_type: ForceType::Repel,
                    hovered: false,
                });
        })
        .insert(UIBar {
            selected_force: ForceType::Passive,
        });
}

fn click_button(
    mut interaction_query: Query<
        (&mut ForceButton, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    mut uibar_query: Query<&mut UIBar>,
) {
    let mut uibar = uibar_query.single_mut();

    for (mut force_button, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                uibar.selected_force = force_button.force_type;
            }
            Interaction::Hovered => {
                force_button.hovered = true;
            }
            Interaction::None => {
                force_button.hovered = false;
            }
        }
    }
}

fn button_color(
    mut button_query: Query<(&ForceButton, &mut BackgroundColor)>,
    mut uibar_query: Query<&mut UIBar>,
) {
    let mut uibar = uibar_query.single_mut();

    for (force_button, mut color) in &mut button_query {
        *color = force_button.color(&mut uibar).into();
    }
}
