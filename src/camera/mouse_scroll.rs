use bevy::{input::mouse::MouseWheel, prelude::*};
pub(super) struct MouseScrollPlugin;

#[derive(Resource, Default)]
pub(super) struct AccumulatedScrolls {
    pub(super) scroll: f32,
}

impl Plugin for MouseScrollPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AccumulatedScrolls::default())
            .add_systems(PreUpdate, accumulate_mouse_scroll);
    }
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
