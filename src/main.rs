mod cells;
mod utils;

use bevy::{prelude::*};
use cells::CellsPlugin;


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

#[derive(Clone, Copy, Debug)]
struct CellPosition {
    x: usize,
    y: usize,
    z: usize,
}

impl Cell {
    fn find_neighbours(self) -> Vec<CellPosition> {
        let mut neighbour_positions: Vec<CellPosition> = vec![];

        for x in (self.cell_position.x - 1)..(self.cell_position.x + 1) {
            for y in (self.cell_position.y - 1)..(self.cell_position.y + 1) {
                for z in (self.cell_position.z - 1)..(self.cell_position.z + 1) {
                    let cell_position = CellPosition {
                        x: x,
                        y: y,
                        z: z,
                    };
                    neighbour_positions.push(cell_position)
                }
            }
        }

        return neighbour_positions
    }
}

#[derive(Debug, Component, Clone)]
struct Cells {
    cells: Vec<Cell>,
}

impl Cells {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let size = width * height * depth;
        let mut cells = Vec::with_capacity(size);

        for x in 0..width {
            for y in 0..height {
                for z in 0..depth {
                    cells.push(Cell { value: 0, cell_position: CellPosition { x, y, z }})
                }
            }
        }

        Self {
            cells,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cells = Cells::new(10, 10, 10);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(2.0, 0.0, 0.1),
            ..default()
        }
    ).insert(cells);
    
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
            for cell in &cells.cells {
                println!("Cell: {:#?}, Neighbours: {:#?}", cell.cell_position, cell.find_neighbours())
            }
        }
        Err(_) => panic!("Cannot load cells"),
    }
}
