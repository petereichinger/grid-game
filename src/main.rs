mod height_grid;

use bevy::{input::keyboard::KeyboardInput, prelude::*, window::ClosingWindow};
use height_grid::HeightGrid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmos)
        .add_systems(Update, close_on_esc)
        .run();
}

fn close_on_esc(
    mut commands: Commands,
    window: Query<(Entity, &Window)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (entity, _) = window.single();
    if keys.just_pressed(KeyCode::Escape) {
        commands.entity(entity).insert(ClosingWindow);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = HeightGrid::new((2, 2), vec![0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    // let mesh = HeightGrid::new((1, 1), vec![0, 0, 0, 0]);
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::WHITE),
        transform: Transform::IDENTITY,
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -4.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
const RED: Color = Color::linear_rgb(1.0, 0.0, 0.0);

fn draw_gizmos(mut gizmos: Gizmos) {
    (0..10).for_each(|x| {
        (0..10).for_each(|y| {
            gizmos.sphere(
                Vec3::new(x as f32, y as f32, 0.0),
                Quat::IDENTITY,
                0.05,
                RED,
            );
        })
    });
}
