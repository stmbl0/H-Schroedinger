use bevy::prelude::*;

use crate::graph2d::tag;

pub fn cleanup_system(mut commands: Commands, q: Query<Entity, With<tag::Graph2DTag>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}
