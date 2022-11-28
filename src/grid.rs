use bevy::prelude::*;

const CELL_WIDTH: f32 = 24.0;
const CELL_HEIGHT: f32 = 24.0;
const GRID_ROWS: usize = 24;
const GRID_COLUMNS: usize = 24;

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
