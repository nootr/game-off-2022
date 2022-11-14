use bevy::prelude::*;

#[derive(Component)]
pub struct UIBar;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.primary();
    let bar_width = window.width() / 12.0;
    let bar_height = window.height();

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(bar_width), Val::Percent(bar_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (window.width() - bar_width) / 2.0,
                0.0,
                0.0,
            )),
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|bar| {
            bar.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(bar_width), Val::Px(bar_width)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        })
        .insert(UIBar);
}
