mod camera;
mod close_on_esc;
mod height_grid;

use bevy::{
    color::palettes::css::LIME,
    core::FrameCount,
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
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // WARN this is a native only feature. It will not work with webgl or webgpu
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Grid Game".into(),
                        name: Some("grid-game".into()),
                        visible: false,
                        ..default()
                    }),
                    ..default()
                }),
            WireframePlugin,
            camera::GameCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc::close_on_esc, make_visible))
        .run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = HeightGrid::new(
        (3, 3),
        vec![
            (0, 0, 0, 0).into(),
            (0, 0, 0, 0).into(),
            (0, 0, 0, 0).into(),
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
