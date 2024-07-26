mod mouse_scroll;

use bevy::{color::palettes::css::*, prelude::*};
use mouse_scroll::{AccumulatedScrolls, MouseScrollPlugin};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MouseScrollPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (move_camera_xy, zoom_camera, rotate_camera));
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct ZRotator;

#[derive(Component)]
struct XRotator;

fn setup(mut commands: Commands) {
    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            MainCamera,
        ))
        .id();
    let x_rotator = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(
                    std::f32::consts::FRAC_PI_4,
                )),
                ..default()
            },
            XRotator,
        ))
        .add_child(camera)
        .id();
    commands
        .spawn((SpatialBundle::default(), ZRotator))
        .add_child(x_rotator);
}

fn move_camera_xy(
    time: Res<Time>,
    mut x_rotator: Query<&mut Transform, With<ZRotator>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut x_trans = x_rotator.single_mut();

    let up = if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        1.0
    } else {
        0.0
    };
    let down = if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        -1.0
    } else {
        0.0
    };

    let left = if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        -1.0
    } else {
        0.0
    };
    let right = if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        1.0
    } else {
        0.0
    };

    let dir = (up + down) * x_trans.up() + (left + right) * x_trans.right();

    x_trans.translation += time.delta_seconds() * dir * 4.0f32;
}

const ZOOM_SPEED: f32 = 50.0;
const MIN_ZOOM: f32 = 25.0;
const MAX_ZOOM: f32 = 100.0;
const MIN_X_ROT: f32 = 0.1745329; // 10 degree
const MAX_X_ROT: f32 = 1.396263; // 80 degree

fn zoom_camera(
    time: Res<Time>,
    mut camera_zoom: Query<&mut Transform, (With<MainCamera>, Without<XRotator>)>,
    mut x_rotator: Query<&mut Transform, (With<XRotator>, Without<MainCamera>)>,
    accumulated_scrolls: Res<AccumulatedScrolls>,
) {
    let mut camera_trans = camera_zoom.single_mut();
    let mut x_rotator = x_rotator.single_mut();

    let mut zoom = camera_trans.translation.z;

    zoom += time.delta_seconds() * ZOOM_SPEED * -1.0 * accumulated_scrolls.scroll;

    zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);

    let zoom_fraction = (zoom - MIN_ZOOM) / (MAX_ZOOM - MIN_ZOOM);

    camera_trans.translation.z = zoom;

    x_rotator.rotation =
        Quat::from_rotation_x(f32::interpolate(&MAX_X_ROT, &MIN_X_ROT, zoom_fraction));
}

fn rotate_camera(
    time: Res<Time>,
    mut z_rotator: Query<&mut Transform, With<ZRotator>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut z_rotator = z_rotator.single_mut();

    let rotate: f32 = if keyboard_input.pressed(KeyCode::KeyQ) {
        -1.0
    } else if keyboard_input.pressed(KeyCode::KeyE) {
        1.0
    } else {
        return;
    };

    z_rotator.rotate_z(time.delta_seconds() * 90.0 * rotate.to_radians());
}
