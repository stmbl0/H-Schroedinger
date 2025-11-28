use bevy::prelude::*;
use bevy_pancam::PanCam;

use crate::graph2d::tag::Graph2DTag;

// Constants
pub const BOHR_RADIUS_UNITS: f32 = 25.0; // 1 a0 = 200 bevy units
pub const MAX_RADIUS_A0: f32 = 100.0; // Maximum radius in Bohr radii
pub const BASE_TICK_SPACING_A0: f32 = 1.0; // Base tick spacing in Bohr radii

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

// Systems
pub fn setup_axis_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(AxisConfig::default());

    // Create X-axis line
    let x_axis_length = MAX_RADIUS_A0 * BOHR_RADIUS_UNITS;
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(x_axis_length, 0.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Graph2DTag,
        AxisTag,
        XAxis,
    ));

    // Create Y-axis line (short, just for reference)
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, -100.0),
            Vec2::new(0.0, 100.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Graph2DTag,
        AxisTag,
    ));

    // Create unit label at the end of x-axis
    commands.spawn((
        Text2d::new("a₀"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
        Transform::from_xyz(x_axis_length + 20.0, 10.0, 2.0),
        Graph2DTag,
        AxisTag,
        XAxisUnitLabel,
    ));
}

pub fn update_axis_ticks_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut axis_config: ResMut<AxisConfig>,
    camera_query: Query<(&Transform, &Projection), (With<Camera>, With<PanCam>)>,
    tick_query: Query<Entity, With<XAxisTick>>,
    label_query: Query<Entity, With<XAxisLabel>>,
) {
    let Ok((camera_transform, projection)) = camera_query.single() else {
        return;
    };

    // Clear existing ticks and labels
    for entity in tick_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in label_query.iter() {
        commands.entity(entity).despawn();
    }

    // Get camera viewport size
    let viewport_width = match projection {
        Projection::Orthographic(ortho) => ortho.area.width(),
        _ => 1000.0, // fallback
    };

    // Calculate appropriate tick spacing
    let camera_x = camera_transform.translation.x;
    let visible_width_world = viewport_width;
    let visible_width_a0 = visible_width_world / BOHR_RADIUS_UNITS;

    // Adjust tick spacing to maintain reasonable number of ticks (aim for 5-15 ticks)
    let target_tick_count = 30.0;
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

    // Calculate tick positions within visible range
    let tick_spacing_world = tick_spacing * BOHR_RADIUS_UNITS;
    let left_edge = camera_x - visible_width_world / 2.0;
    let right_edge = camera_x + visible_width_world / 2.0;

    // Find first tick position
    let first_tick_index = (left_edge.max(0.0) / tick_spacing_world).floor() as i32;
    let last_tick_index = (right_edge.min(axis_config.max_radius_a0 * BOHR_RADIUS_UNITS)
        / tick_spacing_world)
        .ceil() as i32;

    // Calculate tick size based on zoom level
    let tick_height = (visible_width_world * 0.01).clamp(5.0, 20.0);

    // Create ticks and labels
    for i in first_tick_index..=last_tick_index {
        let tick_position_world = i as f32 * tick_spacing_world;
        let tick_position_a0 = tick_position_world / BOHR_RADIUS_UNITS;

        // Skip if outside valid range
        if tick_position_world < 0.0 || tick_position_a0 > axis_config.max_radius_a0 {
            continue;
        }

        // Create tick mark with adaptive height
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

        // Create text label with adaptive formatting
        let label_text = if tick_position_a0 == 0.0 {
            "0".to_string()
        } else if tick_spacing >= 1.0 {
            format!("{:.0}", tick_position_a0)
        } else {
            format!("{:.1}", tick_position_a0)
        };

        // Calculate font size and position based on zoom
        let font_size = (visible_width_world * 0.02).clamp(12.0, 24.0);
        let label_offset = tick_height + font_size * 0.8;

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
