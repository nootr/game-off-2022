use bevy::prelude::*;
use rand::Rng;

use crate::cost::Points;
use crate::force::ForceType;
use crate::game::GameState;
use crate::ghost::Ghost;

#[derive(Component, Default)]
pub struct UIBar {
    pub selected_force: Option<ForceType>,
}

#[derive(Component)]
struct ForceButton {
    force_type: ForceType,
    hovered: bool,
}

impl ForceButton {
    fn color(&self, uibar: &mut UIBar) -> Color {
        let mut color: Color = self.force_type.into();
        color.set_a(self.alpha(uibar));
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
        Some(self.force_type) == uibar.selected_force
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(click_button))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(button_color))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup_uibar));
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
        .spawn((
            NodeBundle {
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
            },
            UIBar::default(),
        ))
        .with_children(|bar| {
            bar.spawn((
                create_ui_button(bar_width, bar_height / 3.0, ForceType::Passive.into()),
                ForceButton {
                    force_type: ForceType::Passive,
                    hovered: false,
                },
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Passive\n$30",
                        TextStyle {
                            font: asset_server.load("fonts/PixeloidSans.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_text_alignment(TextAlignment::TOP_CENTER),
                );
            });

            bar.spawn((
                create_ui_button(bar_width, bar_height / 3.0, ForceType::Attract.into()),
                ForceButton {
                    force_type: ForceType::Attract,
                    hovered: false,
                },
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Attract\n$50",
                        TextStyle {
                            font: asset_server.load("fonts/PixeloidSans.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_text_alignment(TextAlignment::TOP_CENTER),
                );
            });

            bar.spawn((
                create_ui_button(bar_width, bar_height / 3.0, ForceType::Repel.into()),
                ForceButton {
                    force_type: ForceType::Repel,
                    hovered: false,
                },
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Repel\n$60",
                        TextStyle {
                            font: asset_server.load("fonts/PixeloidSans.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_text_alignment(TextAlignment::TOP_CENTER),
                );
            });
        });
}

fn click_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    points: Res<Points>,
    mut interaction_query: Query<
        (&mut ForceButton, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    mut uibar_query: Query<&mut UIBar>,
    ghost_query: Query<Entity, With<Ghost>>,
) {
    let mut uibar = uibar_query.single_mut();
    let mut rng = rand::thread_rng();

    for (mut force_button, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if force_button.force_type.price() <= points.owned {
                    if uibar.selected_force == Some(force_button.force_type) {
                        uibar.selected_force = None;
                    } else {
                        uibar.selected_force = Some(force_button.force_type);
                    }

                    for entity in &ghost_query {
                        commands.entity(entity).despawn();
                    }
                    let texture_atlas = match force_button.force_type {
                        ForceType::Passive => {
                            let render_box_a: bool = rng.gen();
                            let texture_handle = asset_server.load(match render_box_a {
                                true => "sprites/BoxA.png",
                                false => "sprites/BoxB.png",
                            });
                            TextureAtlas::from_grid(
                                texture_handle,
                                Vec2::new(16.0, 16.0),
                                1,
                                1,
                                None,
                                None,
                            )
                        }
                        ForceType::Attract => {
                            let texture_handle =
                                asset_server.load("sprites/spritesheet_coffee.png");
                            TextureAtlas::from_grid(
                                texture_handle,
                                Vec2::new(16.0, 16.0),
                                6,
                                1,
                                None,
                                None,
                            )
                        }
                        ForceType::Repel => {
                            let render_box_a: bool = rng.gen();
                            let texture_handle = asset_server.load(match render_box_a {
                                true => "sprites/Stack_of_work.png",
                                false => "sprites/Stack_of_work_B.png",
                            });
                            TextureAtlas::from_grid(
                                texture_handle,
                                Vec2::new(16.0, 16.0),
                                1,
                                1,
                                None,
                                None,
                            )
                        }
                    };
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);

                    let mut sprite_sheet_bundle = SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            scale: Vec3::splat(4.0 * 1.5),
                            ..default()
                        },
                        ..default()
                    };
                    sprite_sheet_bundle.sprite.color.set_a(0.2);
                    commands.spawn((sprite_sheet_bundle, Ghost));
                }
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
    points: Res<Points>,
    mut button_query: Query<(&ForceButton, &mut BackgroundColor)>,
    mut uibar_query: Query<&mut UIBar>,
) {
    let mut uibar = uibar_query.single_mut();

    for (force_button, mut color) in &mut button_query {
        if force_button.force_type.price() <= points.owned {
            *color = force_button.color(&mut uibar).into();
        } else {
            *color = Color::rgba(1.0, 1.0, 1.0, 0.1).into();
        }
    }
}

fn cleanup_uibar(mut commands: Commands, uibar_query: Query<Entity, With<UIBar>>) {
    let uibar = uibar_query.single();
    commands.entity(uibar).despawn_recursive();
}
