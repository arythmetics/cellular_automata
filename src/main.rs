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
    width: usize,
    height: usize,
    depth: usize,
}

#[derive(Debug, Component)]
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
                    cells.push(Cell { value: 0, width: x, height: y, depth: z })
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
    let cells = query.single_mut();

}

