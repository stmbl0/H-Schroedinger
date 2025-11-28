use bevy::prelude::*;

use crate::graph2d::{axis, tag::Graph2DTag};

// Component to mark plotted points
#[derive(Component)]
pub struct PlottedPoint {
    pub a0_position: f32,
    pub value: f32,
}

// Component to mark function curves
#[derive(Component)]
pub struct FunctionCurve;

// Helper function to convert a value in some arbitrary units to world y coordinates
// Helper function to scale to world y coordinates
pub fn scale_to_world_y(value: f32, max_value: f32, screen_height: f32) -> f32 {
    (value / max_value) * screen_height * 0.4
}

// Helper function to plot a data series with automatic scaling
pub fn plot_data_series(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    data: &[(f32, f32)],
    color: Color,
    zoom: f32,
) {
    if data.len() < 2 {
        return;
    }

    let max_value = data.iter().map(|(_, v)| v.abs()).fold(0.0_f32, f32::max);
    if max_value == 0.0 {
        return;
    }

    let mut world_points = Vec::new();
    for &(a0_pos, value) in data {
        let world_x = axis::a0_to_world(a0_pos, zoom);
        let world_y = scale_to_world_y(value, max_value, 400.0);
        world_points.push(Vec2::new(world_x, world_y));
    }

    for i in 0..world_points.len() - 1 {
        let start = world_points[i];
        let end = world_points[i + 1];

        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(start, end))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 0.5),
            Graph2DTag,
            FunctionCurve,
        ));
    }
}

// Example system to demonstrate coordinate precision
pub fn demonstrate_coordinate_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_query: Query<&crate::graph2d::camera::GraphCamera, With<Camera>>,
) {
    let Ok(_graph_camera) = camera_query.single() else {
        return;
    };
    let zoom = 1.0; // Use fixed zoom for examples

    // Simple test point at origin
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 3.0),
        Graph2DTag,
    ));

    // Test points at known positions (using BASE_BOHR_RADIUS_UNITS directly)
    let test_positions = vec![
        (200.0, 0.0),    // Should be at 1 a0
        (400.0, 50.0),   // Should be at 2 a0
        (1000.0, 100.0), // Should be at 5 a0
    ];

    for (world_x, world_y) in test_positions {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(8.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.8, 0.0))),
            Transform::from_xyz(world_x, world_y, 3.0),
            Graph2DTag,
        ));
    }

    // Plot example function (exponential decay with oscillation)
    let mut curve_points = Vec::new();
    let step_size_a0 = 0.1;
    let max_radius_a0 = 20.0;

    let mut a0 = 0.0;
    while a0 <= max_radius_a0 {
        let world_x = axis::a0_to_world(a0, zoom);
        let value = 100.0 * (1.0 + a0) * (-a0 / 5.0).exp() * (a0 * 2.0).cos();
        let world_y = value;
        curve_points.push(Vec2::new(world_x, world_y));
        a0 += step_size_a0;
    }

    // Create line segments for the curve
    for i in 0..curve_points.len() - 1 {
        let start = curve_points[i];
        let end = curve_points[i + 1];

        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(start, end))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.6, 0.8))),
            Transform::from_xyz(0.0, 0.0, 2.0),
            Graph2DTag,
            FunctionCurve,
        ));
    }

    // Add coordinate system reference lines at key a0 values
    let reference_a0_values = vec![1.0, 5.0, 10.0, 25.0, 50.0];

    for a0_val in reference_a0_values {
        let world_x = axis::a0_to_world(a0_val, zoom);

        // Vertical reference line
        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new(world_x, -200.0),
                Vec2::new(world_x, 200.0),
            ))),
            MeshMaterial2d(materials.add(Color::srgba(0.7, 0.7, 0.7, 0.3))),
            Transform::from_xyz(0.0, 0.0, -1.0), // Behind other elements
            Graph2DTag,
        ));
    }

    // Add horizontal reference line at y=0
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(axis::a0_to_world(100.0, zoom), 0.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgba(0.5, 0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Graph2DTag,
    ));
}
