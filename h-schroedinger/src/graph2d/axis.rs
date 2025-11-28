use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::graph2d::camera::{GraphCamera, get_camera_viewport};
use crate::graph2d::tag::Graph2DTag;

// Constants
pub const BASE_BOHR_RADIUS_UNITS: f32 = 200.0; // Base: 1 a0 = 200 bevy units at zoom 1.0
pub const MAX_RADIUS_A0: f32 = 100.0;
pub const BASE_TICK_SPACING_A0: f32 = 1.0;

// Components
#[derive(Component)]
pub struct AxisTag;

#[derive(Component)]
pub struct XAxis;

#[derive(Component)]
pub struct XAxisTick;

#[derive(Component)]
pub struct XAxisLabel;

#[derive(Component)]
pub struct XAxisUnitLabel;

#[derive(Resource)]
pub struct AxisConfig {
    pub max_radius_a0: f32,
    pub current_tick_spacing_a0: f32,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            max_radius_a0: MAX_RADIUS_A0,
            current_tick_spacing_a0: BASE_TICK_SPACING_A0,
        }
    }
}

// Setup axis system
pub fn setup_axis_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(AxisConfig::default());

    // Create X-axis line - simple unit line that will be scaled by update system
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Graph2DTag,
        AxisTag,
        XAxis,
    ));

    // Create Y-axis line
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, -100.0),
            Vec2::new(0.0, 100.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::default(),
        Graph2DTag,
        AxisTag,
    ));

    // Create unit label at the end of x-axis - will be repositioned by update system
    commands.spawn((
        Text2d::new("a₀"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
        Transform::default(),
        Graph2DTag,
        AxisTag,
        XAxisUnitLabel,
    ));
}

// Update axis system based on camera position and zoom
pub fn update_axis_ticks_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut axis_config: ResMut<AxisConfig>,
    camera_query: Query<(&Transform, &GraphCamera), With<Camera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut x_axis_query: Query<
        &mut Transform,
        (With<XAxis>, Without<Camera>, Without<XAxisUnitLabel>),
    >,
    mut unit_label_query: Query<
        &mut Transform,
        (With<XAxisUnitLabel>, Without<Camera>, Without<XAxis>),
    >,
    tick_query: Query<Entity, With<XAxisTick>>,
    label_query: Query<Entity, With<XAxisLabel>>,
) {
    let Ok((camera_transform, graph_camera)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // Clear existing ticks and labels
    for entity in tick_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in label_query.iter() {
        commands.entity(entity).despawn();
    }

    let window_size = Vec2::new(window.width(), window.height());
    let (viewport_min, viewport_max) = get_camera_viewport(camera_transform, window_size);
    let zoom = graph_camera.current_zoom;

    // Calculate effective units per world coordinate (accounts for zoom)
    let effective_bohr_units = BASE_BOHR_RADIUS_UNITS / zoom;

    // Calculate visible range in a0 units
    let visible_width_world = viewport_max.x - viewport_min.x;
    let visible_width_a0 = visible_width_world / effective_bohr_units;

    // Calculate appropriate tick spacing - show more ticks
    let target_tick_count = 15.0;
    let ideal_spacing = visible_width_a0 / target_tick_count;

    // Round to nearest power of 2 times base spacing
    let mut tick_spacing = BASE_TICK_SPACING_A0;
    while tick_spacing < ideal_spacing {
        tick_spacing *= 2.0;
    }
    if tick_spacing > ideal_spacing * 2.0 && tick_spacing > BASE_TICK_SPACING_A0 {
        tick_spacing /= 2.0;
    }

    axis_config.current_tick_spacing_a0 = tick_spacing;

    // Update X-axis line - scale it to proper length, keep at origin
    let x_axis_length = axis_config.max_radius_a0 * effective_bohr_units;
    if let Ok(mut x_axis_transform) = x_axis_query.single_mut() {
        x_axis_transform.scale = Vec3::new(x_axis_length, 1.0, 1.0);
        x_axis_transform.translation = Vec3::new(0.0, 0.0, 1.0);
    }

    // Update unit label position
    if let Ok(mut unit_transform) = unit_label_query.single_mut() {
        unit_transform.translation = Vec3::new(x_axis_length + 20.0, 10.0, 2.0);
    }

    // Calculate tick positions within visible range
    let tick_spacing_world = tick_spacing * effective_bohr_units;
    let left_edge = viewport_min.x.max(0.0); // Only show positive x values
    let right_edge = viewport_max.x.min(x_axis_length);

    // Find first and last tick indices
    let first_tick_index = (left_edge / tick_spacing_world).floor() as i32;
    let last_tick_index = (right_edge / tick_spacing_world).ceil() as i32;

    // Smaller fixed sizes for ticks and labels
    let tick_height = 10.0;
    let font_size = 12.0;
    let label_offset = tick_height + 10.0;

    // Create ticks and labels
    for i in first_tick_index..=last_tick_index {
        let tick_position_world = i as f32 * tick_spacing_world;
        let tick_position_a0 = tick_position_world / effective_bohr_units;

        // Skip if outside valid range
        if tick_position_world < 0.0 || tick_position_a0 > axis_config.max_radius_a0 {
            continue;
        }

        // Create tick mark with fixed size
        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new(0.0, -tick_height),
                Vec2::new(0.0, tick_height),
            ))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(tick_position_world, 0.0, 2.0),
            Graph2DTag,
            AxisTag,
            XAxisTick,
        ));

        // Create text label with fixed size
        let label_text = if tick_position_a0 == 0.0 {
            "0".to_string()
        } else if tick_spacing >= 1.0 {
            format!("{:.0}", tick_position_a0)
        } else {
            format!("{:.1}", tick_position_a0)
        };

        commands.spawn((
            Text2d::new(label_text),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(Color::srgb(0.3, 0.3, 0.3)),
            Transform::from_xyz(tick_position_world, -label_offset, 2.0),
            Graph2DTag,
            AxisTag,
            XAxisLabel,
        ));
    }
}

// Helper function to convert a0 units to world coordinates (zoom-aware)
pub fn a0_to_world(a0_x: f32, zoom: f32) -> f32 {
    a0_x * (BASE_BOHR_RADIUS_UNITS / zoom)
}

// Helper function to convert world coordinates to a0 units (zoom-aware)
pub fn world_to_a0(world_x: f32, zoom: f32) -> f32 {
    world_x / (BASE_BOHR_RADIUS_UNITS / zoom)
}

// Helper functions for plotting
pub fn plot_value_to_world_y(plot_value: f32, scale_factor: f32) -> f32 {
    plot_value * scale_factor
}

pub fn world_y_to_plot_value(world_y: f32, scale_factor: f32) -> f32 {
    world_y / scale_factor
}
