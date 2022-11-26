use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::{GameState, Volatile};

const CELL_WIDTH: f32 = 24.0;
const CELL_HEIGHT: f32 = 24.0;
const GRID_ROWS: usize = 24;
const GRID_COLUMNS: usize = 24;

#[derive(Debug, Component)]
struct Cell;

pub fn get_coordinates(row: usize, column: usize) -> Vec2 {
    Vec2::new(
        (row as f32 - GRID_ROWS as f32 / 2.0) * CELL_HEIGHT * 4.0,
        (column as f32 - GRID_COLUMNS as f32 / 2.0) * CELL_WIDTH * 4.0,
    )
}

pub fn get_indeces(position: Vec2) -> (usize, usize) {
    (
        ((position.x + CELL_WIDTH * 2.0) / (CELL_WIDTH * 4.0) + GRID_ROWS as f32 / 2.0) as usize,
        ((position.y + CELL_HEIGHT * 2.0) / (CELL_HEIGHT * 4.0) + GRID_COLUMNS as f32 / 2.0)
            as usize,
    )
}

pub fn snap(position: Vec2) -> Vec2 {
    let (x, y) = get_indeces(position);
    get_coordinates(x, y)
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_cell))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(hover_cell))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup_cell));
    }
}

fn setup_cell(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2::new(CELL_WIDTH, CELL_HEIGHT);
    let color = Color::rgb(0.0, 1.0, 0.0);

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::splat(4.0),
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Quad::new(size))).into(),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        },
        Volatile,
        Cell,
    ));
}

fn hover_cell(windows: Res<Windows>, mut cell_query: Query<&mut Transform, With<Cell>>) {
    let window = windows.primary();
    let cursor_position = window.cursor_position();
    let mut cell_transform = cell_query.single_mut();

    if let Some(position) = cursor_position {
        let world_position = position - Vec2::new(window.width(), window.height()) / 2.0;
        cell_transform.translation = snap(world_position).extend(-10.0);
    }
}

fn cleanup_cell(mut commands: Commands, cell_query: Query<Entity, With<Cell>>) {
    let cell = cell_query.single();
    commands.entity(cell).despawn();
}
