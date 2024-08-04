mod camera;
mod close_on_esc;
mod height_grid;
mod input;
mod terrain_editor;

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{GHOST_WHITE, LIME},
    core::FrameCount,
    pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use height_grid::{mesh_builder::RequiresMeshing, HeightGrid};

#[derive(Component, Debug)]
pub struct Terrain;

#[derive(Component, Debug)]
pub struct Ground;

#[derive(Component, Debug)]
pub struct Cliffs;

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
            PhysicsPlugins::default(),
            input::GameInputPlugin,
            height_grid::HeightGridPlugin,
            terrain_editor::TerrainEditorPlugin,
            EguiPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_texture = asset_server.load("textures/grass.png");
    let cliffs_texture = asset_server.load("textures/dirt.png");
    let ground_material = materials.add(StandardMaterial {
        base_color_texture: Some(ground_texture.clone()),
        ..default()
    });

    let cliffs_material = materials.add(StandardMaterial {
        base_color_texture: Some(cliffs_texture.clone()),
        ..default()
    });

    let ground_id = commands
        .spawn((
            PbrBundle {
                material: ground_material,
                transform: Transform::IDENTITY,
                ..default()
            },
            Terrain,
            Ground,
            Wireframe,
            WireframeColor { color: LIME.into() },
            RigidBody::Static,
            Name::new("Ground"),
        ))
        .id();

    let cliffs_id = commands
        .spawn((
            PbrBundle {
                material: cliffs_material,
                transform: Transform::IDENTITY,
                ..default()
            },
            Terrain,
            Cliffs,
            Wireframe,
            WireframeColor {
                color: GHOST_WHITE.into(),
            },
            RigidBody::Static,
            Name::new("Cliffs"),
        ))
        .id();

    let mut height_grid = commands.spawn((
        RequiresMeshing,
        ColliderConstructorHierarchy::new(Some(ColliderConstructor::TrimeshFromMesh)),
        SpatialBundle::default(),
        HeightGrid::new(
            (3, 3),
            [
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
        ),
        Name::new("Height Grid"),
    ));

    height_grid.push_children(&[ground_id, cliffs_id]);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
