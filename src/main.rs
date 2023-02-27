mod cells;
mod utils;

use bevy::prelude::*;
use cells::CellsPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(2.0, 0.0, 0.1),
            ..default()
        }
    );
    
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
            transform: Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}
