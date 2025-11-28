use bevy::{camera::ScalingMode, prelude::*};
use bevy_pancam;

use crate::graph2d::tag;

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        tag::Graph2DTag,
        bevy_pancam::PanCam {
            min_scale: 0.5,
            max_scale: 5.0,
            ..Default::default()
        },
        Projection::Orthographic(OrthographicProjection {
            scale: 2.5,
            ..OrthographicProjection::default_2d()
        }),
    ));

    const X_EXTENT: f32 = 900.;

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(Segment2d::new(
            Vec2::new(0.0, 5000.0),
            Vec2::new(0.0, -5000.0),
        )),
        meshes.add(Polyline2d::new(vec![
            Vec2::new(-50.0, 50.0),
            Vec2::new(0.0, -50.0),
            Vec2::new(50.0, 50.0),
        ])),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                100.0,
                0.0,
            ),
            tag::Graph2DTag,
        ));
    }
}
