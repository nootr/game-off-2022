use bevy::prelude::*;

use crate::game::{GameState, Volatile};

#[derive(Debug, Default, Resource)]
pub struct Points {
    pub owned: f32,
}

#[derive(Component)]
struct PointsText;

pub struct CostPlugin;

impl Plugin for CostPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Points::default())
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(setup_text)
                    .with_system(reset_points),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(increase_points));
    }
}

fn increase_points(
    mut points: ResMut<Points>,
    mut text_query: Query<&mut Text, With<PointsText>>,
    time: Res<Time>,
) {
    let mut text = text_query.single_mut();

    points.owned += time.delta_seconds() * 5.0;
    text.sections[0].value = format!("${}", points.owned as u32);
}

fn reset_points(mut points: ResMut<Points>) {
    points.owned = 30.0;
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "$123",
            TextStyle {
                font: asset_server.load("fonts/PixeloidSans.ttf"),
                font_size: 40.0,
                color: Color::GREEN,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        PointsText,
        Volatile,
    ));
}
