use bevy::prelude::*;

use crate::graph2d::{axis, camera, tag};

pub fn cleanup_system(
    mut commands: Commands,
    q: Query<Entity, With<tag::Graph2DTag>>,
    axis_config: Option<ResMut<axis::AxisConfig>>,
    camera_input: Option<ResMut<camera::CameraInputState>>,
) {
    // Remove axis config resource
    if axis_config.is_some() {
        commands.remove_resource::<axis::AxisConfig>();
    }

    // Remove camera input state resource
    if camera_input.is_some() {
        commands.remove_resource::<camera::CameraInputState>();
    }

    // Remove all tagged entities
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}
