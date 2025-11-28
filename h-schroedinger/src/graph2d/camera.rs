use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::graph2d::tag::Graph2DTag;

// Camera control components
#[derive(Component)]
pub struct GraphCamera {
    pub zoom_speed: f32,
    pub pan_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub current_zoom: f32,
}

impl Default for GraphCamera {
    fn default() -> Self {
        Self {
            zoom_speed: 0.1,
            pan_speed: 1.0,
            min_zoom: 0.1,
            max_zoom: 10.0,
            current_zoom: 1.0,
        }
    }
}

// Input state tracking
#[derive(Resource, Default)]
pub struct CameraInputState {
    pub mouse_position: Vec2,
    pub last_mouse_position: Vec2,
    pub is_dragging: bool,
}

// Setup camera system
pub fn setup_camera_system(mut commands: Commands) {
    commands.insert_resource(CameraInputState::default());

    commands.spawn((Camera2d, GraphCamera::default(), Graph2DTag));
}

// Handle mouse input for panning
pub fn camera_pan_system(
    mut input_state: ResMut<CameraInputState>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &GraphCamera), With<Camera>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((mut camera_transform, _graph_camera)) = camera_query.single_mut() else {
        return;
    };

    // Update mouse position
    if let Some(cursor_pos) = window.cursor_position() {
        input_state.last_mouse_position = input_state.mouse_position;
        input_state.mouse_position = cursor_pos;
    }

    // Handle dragging
    if mouse_buttons.just_pressed(MouseButton::Left) {
        input_state.is_dragging = true;
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        input_state.is_dragging = false;
    }

    // Apply panning
    if input_state.is_dragging && mouse_buttons.pressed(MouseButton::Left) {
        let delta = input_state.mouse_position - input_state.last_mouse_position;

        // Apply movement (inverted for natural feel)
        camera_transform.translation.x -= delta.x;
        camera_transform.translation.y += delta.y; // Inverted Y
    }
}

// Handle mouse wheel for zooming
pub fn camera_zoom_system(
    mut scroll_events: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &mut GraphCamera), With<Camera>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((mut camera_transform, mut graph_camera)) = camera_query.single_mut() else {
        return;
    };

    for scroll in scroll_events.read() {
        let scroll_amount = match scroll.unit {
            MouseScrollUnit::Line => scroll.y,
            MouseScrollUnit::Pixel => scroll.y * 0.01,
        };

        if scroll_amount.abs() < f32::EPSILON {
            continue;
        }

        // Calculate zoom factor
        let zoom_factor = if scroll_amount > 0.0 {
            1.0 - graph_camera.zoom_speed
        } else {
            1.0 + graph_camera.zoom_speed
        };

        let new_zoom = (graph_camera.current_zoom * zoom_factor)
            .clamp(graph_camera.min_zoom, graph_camera.max_zoom);

        if (new_zoom - graph_camera.current_zoom).abs() < f32::EPSILON {
            continue;
        }

        // Get cursor position in window coordinates
        if let Some(cursor_pos) = window.cursor_position() {
            // Convert cursor to world position before zoom
            let window_size = Vec2::new(window.width(), window.height());
            let cursor_world_before =
                screen_to_world_pos(cursor_pos, &camera_transform, window_size);

            // Update zoom
            graph_camera.current_zoom = new_zoom;

            // Convert cursor to world position after zoom
            let cursor_world_after =
                screen_to_world_pos(cursor_pos, &camera_transform, window_size);

            // Adjust camera position to keep cursor point stable
            let world_delta = cursor_world_before - cursor_world_after;
            camera_transform.translation.x += world_delta.x;
            camera_transform.translation.y += world_delta.y;
        } else {
            // Update zoom without cursor adjustment
            graph_camera.current_zoom = new_zoom;
        }
    }
}

// Helper function to convert screen coordinates to world coordinates
fn screen_to_world_pos(screen_pos: Vec2, camera_transform: &Transform, window_size: Vec2) -> Vec2 {
    // Convert screen coordinates to NDC (Normalized Device Coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // Convert NDC to world coordinates (camera scale is always 1.0)
    let world_pos = Vec2::new(ndc.x * window_size.x * 0.5, -ndc.y * window_size.y * 0.5);

    // Add camera position
    world_pos + camera_transform.translation.truncate()
}

// Get camera viewport bounds in world coordinates
pub fn get_camera_viewport(camera_transform: &Transform, window_size: Vec2) -> (Vec2, Vec2) {
    let half_size = window_size * 0.5;
    let camera_pos = camera_transform.translation.truncate();

    let viewport_min = camera_pos - half_size;
    let viewport_max = camera_pos + half_size;

    (viewport_min, viewport_max)
}
