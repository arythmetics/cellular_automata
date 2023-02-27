use std::collections::HashMap;
use bevy::prelude::*;
use bevy::math::{IVec3};

use crate::utils::keep_in_bounds;

pub static VONNEUMAN_NEIGHBOURS: [IVec3; 6] = [
    IVec3::from_array([1, 0, 0]),
    IVec3::from_array([-1, 0, 0]),
    IVec3::from_array([0, 1, 0]),
    IVec3::from_array([0, -1, 0]),
    IVec3::from_array([0, 0, -1]),
    IVec3::from_array([0, 0, 1]),
];

static BOUNDING_SIZE: i32 = 25;
static SURVIVAL_RULE: [i32; 3] = [2, 6, 9];
static BIRTH_RULE: [i32; 5] = [4, 6, 8, 9, 10];
static STATES: u8 = 10;
static COLOR_METHOD: Color = Color::RED;
static NEIGHBOUR_METHOD: [IVec3; 6] = VONNEUMAN_NEIGHBOURS;

#[derive(Debug)]
pub struct CellState {
    value: u8,
    neighbours: u8,
    dist_to_center: f32,
}

impl CellState {
    pub fn new(value: u8, neighbours: u8, dist_to_center: f32) -> Self {
        CellState {
            value,
            neighbours,
            dist_to_center,
        }
    }
}

#[derive(Resource)]
struct Cells {
    states: HashMap<IVec3, CellState>,
    neighbours: HashMap<IVec3, u8>,
}

impl Cells {
    pub fn new() -> Self {
        Cells {
            states: HashMap::new(),
            neighbours: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {
        self.calculate_neighbours();
    }

    pub fn calculate_neighbours(&mut self) {
        for (cell_pos, cell) in self.states.iter() {
            println!("{}", cell_pos);
            if cell.value == STATES {
                for direction in &NEIGHBOUR_METHOD[..] {
                    let mut neighbour_pos = *cell_pos + *direction;
                    keep_in_bounds(BOUNDING_SIZE, &mut neighbour_pos);
                    if !self.neighbours.contains_key(&neighbour_pos) {
                        self.neighbours.insert(neighbour_pos, 0);
                    }
                    let neighbour = self.neighbours.get_mut(&neighbour_pos).unwrap();
                    *neighbour += 1;
                    println!("{}", neighbour);
                }
            }
        }
    }
}

fn tick_cell(
    mut cells: ResMut<Cells>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Right) {
        cells.tick();
    }
}

pub struct CellsPlugin;
impl Plugin for CellsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let cells = Cells::new();
        app.insert_resource(cells)
            .add_system(tick_cell);
    }
}