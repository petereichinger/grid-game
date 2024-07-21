use bevy::{input::mouse::MouseWheel, prelude::*, render::extract_resource::ExtractResource};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AccumulatedScrolls::default())
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, accumulate_mouse_scroll)
            .add_systems(Update, (move_camera_xy, zoom_camera));
    }
}

#[derive(Resource, Default)]
struct AccumulatedScrolls {
    scroll: f32,
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
    mut x_rotator: Query<&mut Transform, With<XRotator>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut x_trans = x_rotator.single_mut();

    let up = if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        Vec3::Y
    } else {
        Vec3::ZERO
    };
    let down = if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        Vec3::NEG_Y
    } else {
        Vec3::ZERO
    };

    let left = if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        Vec3::NEG_X
    } else {
        Vec3::ZERO
    };
    let right = if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        Vec3::X
    } else {
        Vec3::ZERO
    };

    let dir = up + down + left + right;

    x_trans.translation += time.delta_seconds() * dir * 4.0f32;
}

fn zoom_camera(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    accumulated_scrolls: Res<AccumulatedScrolls>,
) {
    let mut camera_trans = camera.single_mut();
    let mut zoom = camera_trans.translation.z;

    zoom += time.delta_seconds() * 50.0 * -1.0 * accumulated_scrolls.scroll;

    zoom = zoom.clamp(25.0, 100.0);

    camera_trans.translation.z = zoom;
}

fn accumulate_mouse_scroll(
    mut scrolls: EventReader<MouseWheel>,
    mut accumulated_scrolls: ResMut<AccumulatedScrolls>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let scroll_sum: f32 = scrolls
        .read()
        .map(|scroll| match scroll.unit {
            MouseScrollUnit::Line => 8.0 * scroll.y,
            MouseScrollUnit::Pixel => scroll.y,
        })
        .sum();

    accumulated_scrolls.scroll = scroll_sum;
}
