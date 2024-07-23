mod camera;
mod close_on_esc;
mod height_grid;

use bevy::{
    color::palettes::css::LIME,
    pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};

use height_grid::HeightGrid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
            camera::GameCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = HeightGrid::new(
        (3, 3),
        vec![
            (0, 1, 0, 0).into(),
            (1, 1, 0, 0).into(),
            (1, 0, 0, 0).into(),
            (0, 1, 0, 1).into(),
            (1, 1, 1, 1).into(),
            (1, 0, 1, 0).into(),
            (0, 0, 0, 1).into(),
            (0, 0, 1, 1).into(),
            (0, 0, 1, 0).into(),
        ],
    );
    let mesh = height_grid::mesh_builder::Builder::new(&grid);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::WHITE),
            transform: Transform::IDENTITY,
            ..default()
        },
        Wireframe,
        WireframeColor { color: LIME.into() },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
