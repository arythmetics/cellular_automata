use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    
    let initial_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    
    commands.spawn(
        PbrBundle {
            mesh: initial_cube,
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        });
    
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}
