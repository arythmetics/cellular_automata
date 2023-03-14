mod cells;
mod utils;

use std::collections::HashMap;

use bevy::{prelude::*};
use bevy::math::IVec3;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    value: u8,
    cell_position: CellPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CellPosition {
    x: usize,
    y: usize,
    z: usize,
}

impl Cell {
    
}

#[derive(Debug, Component, Clone)]
struct Cells {
    cell_index: HashMap<CellPosition, Cell>,
    cell_map: IVec3,
}

impl Cells {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let size: usize = width * height * depth;
        let mut cells = HashMap::with_capacity(size);

        for x in 0..width {
            for y in 0..height {
                for z in 0..depth {
                    cells.insert(CellPosition { x, y, z }, Cell { value: 0, cell_position: CellPosition { x, y, z }});
                }
            }
        }

        let cell_map = IVec3 { x: width, y: height, z: size };

        Self {
            cell_index: cells,
        }
    }

    fn find_neighbours(self, cell_position: &CellPosition) -> Vec<Cell> {
        let mut neighbour_cells: Vec<Cell> = vec![];

        for x in (cell_position.x - 1)..(cell_position.x + 1) {
            for y in (cell_position.y - 1)..(cell_position.y + 1) {
                for z in (cell_position.z - 1)..(cell_position.z + 1) {
                    let cell_position = CellPosition {
                        x: x,
                        y: y,
                        z: z,
                    };
                    let neighbour_cell = self.cell_index.get(&cell_position).unwrap().clone();
                    neighbour_cells.push(neighbour_cell)
                }
            }
        }
        return neighbour_cells
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell_map = Cells::new(10, 10, 10);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(2.0, 0.0, 0.1),
            ..default()
        }
    ).insert(cell_map);
    
    commands.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 100.,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default() 
        }
    );
    
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}

fn update(mut query: Query<&mut Cells>) {
    let cells = query.get_single_mut();

    match cells {
        Ok(cells) => {
            for (position, _cell) in &cells.cell_index {
                println!("Cell: {:#?}, Neighbours: {:#?}", position, cell_map.find_neighbours(&position))
            }
        }
        Err(_) => panic!("Cannot load cells"),
    }
}
