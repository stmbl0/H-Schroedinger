use bevy::prelude::*;

use crate::graph2d::{axis, tag};

pub fn cleanup_system(
    mut commands: Commands,
    q: Query<Entity, With<tag::Graph2DTag>>,
    axis_config: Option<ResMut<axis::AxisConfig>>,
) {
    // Remove axis config resource
    if axis_config.is_some() {
        commands.remove_resource::<axis::AxisConfig>();
    }

    // Remove all tagged entities
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}
