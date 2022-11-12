//! Tools for Pathfinding.
//!
//! Everytime the solid, static objects are changed (a new level has been initialized, a wall has
//! been removed), a pathfinding algorithm is executed with results in a vector map. Enemies use
//! this vector map to determine their optimal path to the tower.
//!
//! The algorithm is as follows:
//!
//! 1. An N by M grid of Cell objects is initialized.
//! 2. For each solid and static object, a `solid` flag is set for each corresponding Cell.
//! 3. Starting with the Cell corresponding with the Tower, the movement property of each
//!    non-solid neighbouring Cell is set towards the root cell.
//! 4. Step 3 is repeated for each non-solid neighbouring Cell (first N/E/S/W, then diagonally).
//!

use bevy::{ecs::query::QuerySingleError, prelude::*, time::FixedTimestep};
use log::debug;
use rand::Rng;
use std::{cmp, fmt};

use crate::physics::{Collider, Solid};
use crate::tower::Tower;

const GRID_ROWS: usize = 30;
const GRID_COLUMNS: usize = 30;

#[derive(Debug)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Debug, Default)]
pub struct VectorField {
    cells: [[Cell; GRID_COLUMNS]; GRID_ROWS],
    width: f32,
    height: f32,
}

impl fmt::Display for VectorField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                match self.cells[row][column].movement {
                    Some(Movement::Up) => s.push_str("⬆️ "),
                    Some(Movement::Down) => s.push_str("⬇️ "),
                    Some(Movement::Left) => s.push_str("⬅️ "),
                    Some(Movement::Right) => s.push_str("➡️ "),
                    Some(Movement::UpLeft) => s.push_str("↖️ "),
                    Some(Movement::UpRight) => s.push_str("↗️ "),
                    Some(Movement::DownLeft) => s.push_str("↙️ "),
                    Some(Movement::DownRight) => s.push_str("↘️ "),
                    None => s.push('🟥'),
                }
            }
            s.push('\n');
        }

        write!(f, "{}", &s)
    }
}

impl VectorField {
    pub fn get_direction(&self, from: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();

        let (row, column) = self.position_to_index(from.truncate());
        let cell: &Cell = &self.cells[row][column];

        match &cell.movement {
            Some(Movement::Up) => Vec3::new(0.0, -1.0, 0.0),
            Some(Movement::Down) => Vec3::new(0.0, 1.0, 0.0),
            Some(Movement::Left) => Vec3::new(-1.0, 0.0, 0.0),
            Some(Movement::Right) => Vec3::new(1.0, 0.0, 0.0),
            Some(Movement::UpRight) => Vec3::new(0.7, -0.7, 0.0),
            Some(Movement::UpLeft) => Vec3::new(-0.7, -0.7, 0.0),
            Some(Movement::DownRight) => Vec3::new(0.7, 0.7, 0.0),
            Some(Movement::DownLeft) => Vec3::new(-0.7, 0.7, 0.0),
            None => Vec3::new(rng.gen_range(-0.7..0.7), rng.gen_range(-0.7..0.7), 0.0),
        }
    }

    /// Generate the VectorField.
    ///
    /// Should only be executed _after_ the solid/tower cells have been set.
    fn generate(&mut self) {
        let mut queue = Vec::new();

        // Add the tower cells to the queue.
        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                let cell: &Cell = &self.cells[row][column];
                if cell.tower {
                    queue.push((row, column));
                }
            }
        }

        // For each cell on the queue, set the direction of each neighbour towards it.
        while !queue.is_empty() {
            let (row, column) = queue
                .pop()
                .expect("queue should not be empty at this point");

            if row > 0 {
                let neighbour_n: &mut Cell = &mut self.cells[row - 1][column];
                if neighbour_n.movement.is_none() && !neighbour_n.solid {
                    neighbour_n.movement = Some(Movement::Down);
                    queue.insert(0, (row - 1, column));
                }

                if column > 0 {
                    let neighbour_nw: &mut Cell = &mut self.cells[row - 1][column - 1];
                    if neighbour_nw.movement.is_none() && !neighbour_nw.solid {
                        neighbour_nw.movement = Some(Movement::DownRight);
                        queue.insert(0, (row - 1, column - 1));
                    }
                }

                if column < GRID_COLUMNS - 1 {
                    let neighbour_ne: &mut Cell = &mut self.cells[row - 1][column + 1];
                    if neighbour_ne.movement.is_none() && !neighbour_ne.solid {
                        neighbour_ne.movement = Some(Movement::DownLeft);
                        queue.insert(0, (row - 1, column + 1));
                    }
                }
            }

            if row < GRID_ROWS - 1 {
                let neighbour_s: &mut Cell = &mut self.cells[row + 1][column];
                if neighbour_s.movement.is_none() && !neighbour_s.solid {
                    neighbour_s.movement = Some(Movement::Up);
                    queue.insert(0, (row + 1, column));
                }

                if column > 0 {
                    let neighbour_sw: &mut Cell = &mut self.cells[row + 1][column - 1];
                    if neighbour_sw.movement.is_none() && !neighbour_sw.solid {
                        neighbour_sw.movement = Some(Movement::UpRight);
                        queue.insert(0, (row + 1, column - 1));
                    }
                }

                if column < GRID_COLUMNS - 1 {
                    let neighbour_se: &mut Cell = &mut self.cells[row + 1][column + 1];
                    if neighbour_se.movement.is_none() && !neighbour_se.solid {
                        neighbour_se.movement = Some(Movement::UpLeft);
                        queue.insert(0, (row + 1, column + 1));
                    }
                }
            }

            if column > 0 {
                let neighbour_w: &mut Cell = &mut self.cells[row][column - 1];
                if neighbour_w.movement.is_none() && !neighbour_w.solid {
                    neighbour_w.movement = Some(Movement::Right);
                    queue.insert(0, (row, column - 1));
                }
            }

            if column < GRID_COLUMNS - 1 {
                let neighbour_e: &mut Cell = &mut self.cells[row][column + 1];
                if neighbour_e.movement.is_none() && !neighbour_e.solid {
                    neighbour_e.movement = Some(Movement::Left);
                    queue.insert(0, (row, column + 1));
                }
            }
        }
    }

    pub fn reset(&mut self) {
        let field = Self::default();
        self.cells = field.cells;
    }

    fn position_to_index(&self, position: Vec2) -> (usize, usize) {
        let row = GRID_ROWS as f32 * ((position.y / self.height) + 0.5);
        let column = GRID_COLUMNS as f32 * ((position.x / self.width) + 0.5);
        (
            cmp::min(cmp::max(row as usize, 0), GRID_ROWS - 1),
            cmp::min(cmp::max(column as usize, 0), GRID_COLUMNS - 1),
        )
    }

    fn get_region_indices(
        &mut self,
        upperleft: Vec2,
        lowerright: Vec2,
    ) -> (Vec<usize>, Vec<usize>) {
        let (row_min, column_min) = self.position_to_index(upperleft);
        let (row_max, column_max) = self.position_to_index(lowerright);

        (
            (row_min..=row_max).collect(),
            (column_min..=column_max).collect(),
        )
    }
}

#[derive(Debug, Default)]
struct Cell {
    solid: bool,
    tower: bool,
    movement: Option<Movement>,
}

pub struct VectorFieldPlugin;

impl Plugin for VectorFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(update_vector_field),
        )
        .add_startup_system(create_vector_field)
        .add_system(update_screen_dimensions);
    }
}

fn create_vector_field(mut commands: Commands) {
    commands.insert_resource(VectorField::default());
}

fn update_screen_dimensions(mut vector_field: ResMut<VectorField>, windows: Res<Windows>) {
    let window = windows.primary();
    vector_field.width = window.width() as f32;
    vector_field.height = window.height() as f32;
}

fn update_vector_field(
    mut vector_field: ResMut<VectorField>,
    tower_query: Query<(&Collider, &Transform), With<Tower>>,
    solid_query: Query<(&Collider, &Transform), (With<Solid>, Without<Tower>)>,
) {
    vector_field.reset();

    // Set tower flags
    match tower_query.get_single() {
        Ok((tower_collider, tower_transform)) => {
            let tower_upperleft =
                tower_transform.translation.truncate() - (tower_collider.hit_box / 1.8);
            let tower_lowerright =
                tower_transform.translation.truncate() + (tower_collider.hit_box / 1.8);

            let (rows, columns) =
                vector_field.get_region_indices(tower_upperleft, tower_lowerright);
            for row in &rows {
                for column in &columns {
                    let cell: &mut Cell = &mut vector_field.cells[*row][*column];
                    cell.solid = true;
                    cell.tower = true;
                }
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {}
        Err(QuerySingleError::MultipleEntities(_)) => error!("Multiple towers!"),
    }

    // Set solid flags
    for (solid_collider, solid_transform) in &solid_query {
        let upperleft = solid_transform.translation.truncate() - (solid_collider.hit_box / 1.8);
        let lowerright = solid_transform.translation.truncate() + (solid_collider.hit_box / 1.8);

        let (rows, columns) = vector_field.get_region_indices(upperleft, lowerright);
        for row in &rows {
            for column in &columns {
                let cell: &mut Cell = &mut vector_field.cells[*row][*column];
                cell.solid = true;
            }
        }
    }

    vector_field.generate();
    debug!("{}", *vector_field);
}
