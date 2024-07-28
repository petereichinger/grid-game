mod camera;
mod close_on_esc;
mod height_grid;
mod input;

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{GHOST_WHITE, LIME, WHITE},
    core::FrameCount,
    input::mouse::MouseMotion,
    pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};

use camera::MainCamera;
use height_grid::{
    mesh_builder::{self, HeightGridMeshes},
    HeightGrid,
};
use input::CurrentMousePos;

#[derive(Component)]
struct Terrain;

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
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc::close_on_esc, make_visible))
        .add_systems(Update, raycast)
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

fn raycast(
    spatial_query: SpatialQuery,
    mut gizmos: Gizmos,
    terrain: Query<(&Terrain)>,
    main_camera: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    mouse_position: Res<CurrentMousePos>,
) {
    let (t, camera) = main_camera
        .get_single()
        .expect("only one main camera allowed");
    // mouse_motion_events.Some
    // camera.ndc_to_world(t, )
    // let origin = t.translation();
    let ray = camera
        .viewport_to_world(t, mouse_position.position)
        .unwrap_or(Ray3d::new(t.translation(), *t.forward()));
    // let direction = t.forward();

    if let Some(ray_hit_data) = spatial_query.cast_ray_predicate(
        ray.origin,
        ray.direction,
        f32::MAX,
        true,
        SpatialQueryFilter::default(),
        &|entity| terrain.get(entity).is_ok(),
    ) {
        let impact = ray.origin + ray_hit_data.time_of_impact * ray.direction;
        gizmos.sphere(impact, Quat::IDENTITY, 0.25, WHITE);
        // info!("{:?}", ray_hit_data);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = HeightGrid::new(
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
    );

    let HeightGridMeshes { ground, cliffs } = mesh_builder::build(&grid);
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

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ground),
            material: ground_material,
            transform: Transform::IDENTITY,
            ..default()
        },
        Terrain,
        Wireframe,
        WireframeColor { color: LIME.into() },
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(cliffs),
            material: cliffs_material,
            transform: Transform::IDENTITY,
            ..default()
        },
        Terrain,
        Wireframe,
        WireframeColor {
            color: GHOST_WHITE.into(),
        },
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
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
